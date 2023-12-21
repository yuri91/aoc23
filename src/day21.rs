use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;
use pom::parser::*;
use std::collections::HashSet;

fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'.').map(|_| Tile::Plot) | sym(b'#').map(|_| Tile::Rock) | sym(b'S').map(|_| Tile::Start)
}

fn row<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}

fn map<'a>() -> Parser<'a, u8, Vec<Vec<Tile>>> {
    row().repeat(1..) - end()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Plot,
    Rock,
    Start,
}

#[aoc_generator(day21)]
fn input_gen(input: &[u8]) -> Result<Array2<Tile>> {
    let data = map().parse(input)?;
    let shape = (data.len(), data[0].len());
    let flat = data.into_iter().flatten().collect();
    Ok(Array2::from_shape_vec(shape, flat)?)
}

fn rem((y, x): (i64, i64), h: usize, w: usize) -> (i64, i64) {
    (y.rem_euclid(h as i64), x.rem_euclid(w as i64))
}
fn wrap((y, x): (i64, i64), h: usize, w: usize) -> (usize, usize) {
    (
        y.rem_euclid(h as i64) as usize,
        x.rem_euclid(w as i64) as usize,
    )
}

fn neighbours(map: &Array2<Tile>, p: (i64, i64)) -> impl Iterator<Item = (i64, i64)> + '_ {
    let &[h, w] = map.shape() else { unreachable!() };
    [
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
    ]
    .into_iter()
    .filter(move |&p| map[wrap(p, h, w)] != Tile::Rock)
}

fn tiles_within_steps(map: &Array2<Tile>, steps: i64, infinite: bool) -> i64 {
    let &[h, w] = map.shape() else { unreachable!() };
    let s = map
        .indexed_iter()
        .find(|(_, &t)| t == Tile::Start)
        .map(|(p, _)| (p.0 as i64, p.1 as i64))
        .unwrap();
    let mut visited = HashSet::new();
    let mut cur = vec![s];
    let mut cnt = 0;
    let even = (steps + 1) % 2 == 0;
    for i in 0..steps {
        let mut next = vec![];
        for c in cur {
            for n in neighbours(map, c) {
                if !infinite && rem(c, h, w) != c {
                    continue;
                }
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n);
                next.push(n);
                if (i % 2 == 0 && even) || (i % 2 == 1 && !even) {
                    cnt += 1;
                }
            }
        }
        cur = next;
    }
    cnt
}

#[aoc(day21, part1)]
fn part1(input: &Array2<Tile>) -> i64 {
    tiles_within_steps(input, 64, false)
}
#[aoc(day21, part2)]
fn part2(input: &Array2<Tile>) -> i64 {
    let &[h, w] = input.shape() else {
        unreachable!()
    };
    assert!(h == w, "not a square");
    let steps = 26501365;
    let m = steps % h as i64;
    let n0 = tiles_within_steps(input, m, true);
    let n1 = tiles_within_steps(input, m + (h as i64), true);
    let n2 = tiles_within_steps(input, m + 2 * (h as i64), true);
    let d1_1 = n1 - n0;
    let d1_2 = n2 - n1;
    let d2 = d1_2 - d1_1;
    let c = n0;
    let a = d2 / 2;
    let b = d1_1 - a;
    let f = |n| a * n * n + b * n + c;
    f((steps - m) / (h as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn part1_example() {
        assert_eq!(
            tiles_within_steps(&input_gen(EXAMPLE).unwrap(), 6, false),
            16
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            tiles_within_steps(&input_gen(EXAMPLE).unwrap(), 50, true),
            1594
        );
    }
}
