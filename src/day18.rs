use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use pom::parser::*;

fn dir<'a>() -> Parser<'a, u8, Dir> {
    sym(b'R').map(|_| Dir::Right)
        | sym(b'L').map(|_| Dir::Left)
        | sym(b'D').map(|_| Dir::Down)
        | sym(b'U').map(|_| Dir::Up)
}
fn dec<'a>() -> Parser<'a, u8, u64> {
    ((one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0'))
        .collect()
        .convert(std::str::from_utf8)
        .convert(|s| s.parse())
}
fn hex<'a>() -> Parser<'a, u8, u64> {
    (one_of(b"0123456789abcdef").repeat(1..))
        .collect()
        .convert(std::str::from_utf8)
        .convert(|s| u64::from_str_radix(s, 16))
}

fn row<'a>() -> Parser<'a, u8, Step> {
    (dir() - sym(b' ') + dec() - sym(b' ') - sym(b'(') - sym(b'#') + hex()
        - sym(b')')
        - (sym(b'\n').discard() | end()))
    .map(|((dir, count), color)| Step { dir, count, color })
}
fn map<'a>() -> Parser<'a, u8, Vec<Step>> {
    row().repeat(1..) - end()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
    dir: Dir,
    count: u64,
    color: u64,
}

#[aoc_generator(day18)]
fn input_gen(input: &[u8]) -> Result<Vec<Step>> {
    Ok(map().parse(input)?)
}

fn get_lagoon_size(steps: &[(Dir, u64)]) -> u64 {
    let mut prev: (i64, i64) = (0, 0);
    let mut inside = 0;
    let mut border = 2;
    for &(dir, count) in steps {
        let cur = match dir {
            Dir::Left => (prev.0, prev.1 - count as i64),
            Dir::Up => (prev.0 - count as i64, prev.1),
            Dir::Right => (prev.0, prev.1 + count as i64),
            Dir::Down => (prev.0 + count as i64, prev.1),
        };
        inside += (prev.0 + cur.0) * (prev.1 - cur.1);
        border += count;
        prev = cur;
    }
    (inside.unsigned_abs() + border) / 2
}

#[aoc(day18, part1)]
fn part1(input: &[Step]) -> u64 {
    let steps: Vec<_> = input
        .iter()
        .map(|&Step { dir, count, .. }| (dir, count))
        .collect();
    get_lagoon_size(&steps)
}

#[aoc(day18, part2)]
fn part2(input: &[Step]) -> u64 {
    let steps: Vec<_> = input
        .iter()
        .map(|&Step { color, .. }| {
            let count = color >> 4;
            let dir = match color & 0xf {
                0 => Dir::Right,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Up,
                _ => {
                    unreachable!();
                }
            };
            (dir, count)
        })
        .collect();
    get_lagoon_size(&steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 62);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 952408144115);
    }
}
