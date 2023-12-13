use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::{Array2, Axis};
use pom::parser::*;

fn tile<'a>() -> Parser<'a, u8, u8> {
    sym(b'.').map(|_| 0) | sym(b'#').map(|_| 1)
}
fn row<'a>() -> Parser<'a, u8, Vec<u8>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}

fn map<'a>() -> Parser<'a, u8, Map> {
    row().repeat(1..).map(|data| {
        let shape = (data.len(), data[0].len());
        let flat = data.into_iter().flatten().collect_vec();
        Map {
            data: Array2::from_shape_vec(shape, flat).unwrap(),
        }
    })
}

fn maps<'a>() -> Parser<'a, u8, Vec<Map>> {
    list(map(), sym(b'\n'))
}

#[derive(Debug, Clone)]
struct Map {
    data: Array2<u8>,
}

#[aoc_generator(day13)]
fn input_gen(input: &[u8]) -> Result<Vec<Map>> {
    Ok(maps().parse(input)?)
}

fn find_lane_reflection(m: &Array2<u8>, axis: usize, smudges: u32) -> i64 {
    let len = m.axes().nth(axis).unwrap().len as i64;
    'outer: for i in 0..(len - 1) {
        let mut i1 = i;
        let mut i2 = i + 1;
        let mut cur_smudges = 0;
        while i1 >= 0 && i2 < len {
            let diff = m.index_axis(Axis(axis), i1 as usize).map(|&t| t as u32)
                ^ m.index_axis(Axis(axis), i2 as usize).map(|&t| t as u32);
            cur_smudges += diff.sum();
            if cur_smudges > smudges {
                continue 'outer;
            }
            i1 -= 1;
            i2 += 1;
        }
        if cur_smudges == smudges {
            return i + 1;
        }
    }
    0
}

#[aoc(day13, part1)]
fn part1(input: &[Map]) -> i64 {
    let mut res = 0;
    for m in input {
        let mut i = find_lane_reflection(&m.data, 1, 0);
        if i == 0 {
            i = find_lane_reflection(&m.data, 0, 0) * 100;
        }
        res += i;
    }
    res
}

#[aoc(day13, part2)]
fn part2(input: &[Map]) -> i64 {
    let mut res = 0;
    for m in input {
        let mut i = find_lane_reflection(&m.data, 1, 1);
        if i == 0 {
            i = find_lane_reflection(&m.data, 0, 1) * 100;
        }
        res += i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 405);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 400);
    }
}
