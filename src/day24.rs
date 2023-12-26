use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Matrix2, Matrix6, Vector2, Vector3, Vector6};
use pom::parser::*;

fn integer<'a>() -> Parser<'a, u8, i64> {
    let unsigned = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    let signed = sym(b'-').opt() + unsigned;
    signed
        .collect()
        .convert(std::str::from_utf8)
        .convert(|s| s.parse())
}

fn spaced_sym<'a>(s: u8) -> Parser<'a, u8, ()> {
    (sym(b' ').repeat(0..) - sym(s) - sym(b' ').repeat(0..)).discard()
}

fn coords<'a>() -> Parser<'a, u8, Vector3<f64>> {
    (integer() - spaced_sym(b',') + integer() - spaced_sym(b',') + integer())
        .map(|((x, y), z)| Vector3::new(x as f64, y as f64, z as f64))
}

fn hail<'a>() -> Parser<'a, u8, Hail> {
    (coords() - spaced_sym(b'@') + coords() - (sym(b'\n').discard() | end()))
        .map(|(pos, vel)| Hail { pos, vel })
}

fn hails<'a>() -> Parser<'a, u8, Vec<Hail>> {
    hail().repeat(1..) - end()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Hail {
    pos: Vector3<f64>,
    vel: Vector3<f64>,
}

fn collision_2d(h1: &Hail, h2: &Hail) -> Option<Vector2<f64>> {
    let p1 = h1.pos.xy();
    let p2 = h2.pos.xy();
    let v1 = h1.vel.xy();
    let v2 = h2.vel.xy();
    if v1 == v2 {
        if p1 == p2 {
            return Some(p1);
        } else {
            return None;
        }
    }
    let m = Matrix2::from_columns(&[v1, -v2]);
    let b = p2 - p1;
    let Some(m_inv) = m.try_inverse() else {
        return None;
    };
    let a = m_inv * b;
    if a > Vector2::zeros() {
        Some(v1 * a.x + p1)
    } else {
        None
    }
}

//(p0 - p[i]) x (v0 - v[i]) == 0
// p0 x v0 - p0 x v[i] - p[i] x v0 + p[i] x v[i] = 0
// p0 x v0 = p0 x v[i] + p[i] x v0 - p[i] x v[i]
// p0 x v1 + p1 x v0 - p1 x v1 = p0 x v2 + p2 x v0 - p2 x v2
// p0 x (v1 - v2) + v0 x (p2 - p1) - (p1 x v1 + p2 x v2) = 0
//
// V'p0 + P'v0 + C' = 0
// V''p0 + P''v0 + C'' = 0
// [ V'  P'    [ p0
//   V'' P'' ]   v0 ]
//
//
//  [a*p0 + b*v0]
//  [c*p0 + d*v0]

fn solve(h1: Hail, h2: Hail, h3: Hail) -> Vector3<f64> {
    let v1 = (h1.vel - h2.vel).cross_matrix();
    let v2 = (h1.vel - h3.vel).cross_matrix();
    let p1 = (h2.pos - h1.pos).cross_matrix();
    let p2 = (h3.pos - h1.pos).cross_matrix();
    let c1 = -h1.pos.cross(&h1.vel) + h2.pos.cross(&h2.vel);
    let c2 = -h1.pos.cross(&h1.vel) + h3.pos.cross(&h3.vel);
    let mut c: Vector6<f64> = Vector6::zeros();
    let mut c1v = c.fixed_view_mut::<3, 1>(0, 0);
    c1v += c1;
    let mut c2v = c.fixed_view_mut::<3, 1>(3, 0);
    c2v += c2;
    let mut m = Matrix6::zeros();
    let mut m11v = m.fixed_view_mut::<3, 3>(0, 0);
    m11v += v1;
    let mut m21v = m.fixed_view_mut::<3, 3>(3, 0);
    m21v += v2;
    let mut m12v = m.fixed_view_mut::<3, 3>(0, 3);
    m12v += p1;
    let mut m22v = m.fixed_view_mut::<3, 3>(3, 3);
    m22v += p2;
    let m_inv = m.try_inverse().unwrap();
    let x = m_inv * c;
    x.fixed_view::<3, 1>(0, 0).into()
}

fn collisions_2d_within(hails: &[Hail], min: Vector2<f64>, max: Vector2<f64>) -> usize {
    let mut ret = 0;
    for pair in hails.iter().combinations(2) {
        let [h1, h2] = pair.as_slice() else {
            unreachable!()
        };
        if let Some(c) = collision_2d(h1, h2) {
            if c >= min && c <= max {
                ret += 1;
            }
        }
    }
    ret
}

#[aoc_generator(day24)]
fn input_gen(input: &[u8]) -> Result<Vec<Hail>> {
    Ok(hails().parse(input)?)
}

#[aoc(day24, part1)]
fn part1(input: &[Hail]) -> usize {
    collisions_2d_within(
        input,
        Vector2::from_element(200000000000000.),
        Vector2::from_element(400000000000000.),
    )
}

#[aoc(day24, part2)]
fn part2(input: &[Hail]) -> usize {
    let avg: Vector3<f64> =
        input.iter().map(|h| h.pos).sum::<Vector3<f64>>() / (input.len() as f64);
    let input: Vec<_> = input
        .iter()
        .copied()
        .map(|mut h| {
            h.pos -= avg;
            h
        })
        .collect();
    let x = solve(input[0], input[1], input[2]) + avg;
    x.into_iter().map(|i| i.round() as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn part1_example() {
        assert_eq!(
            collisions_2d_within(
                &input_gen(EXAMPLE).unwrap(),
                Vector2::from_element(7.),
                Vector2::from_element(27.)
            ),
            2
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 47);
    }
}
