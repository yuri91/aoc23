use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pom::parser::*;

fn integer<'a>() -> Parser<'a, u8, i64> {
    let unsigned = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    let signed = sym(b'-').opt() + unsigned;
    signed
        .collect()
        .convert(std::str::from_utf8)
        .convert(|s| s.parse())
}

#[derive(Debug)]
struct Readings {
    lines: Vec<Vec<i64>>,
}

#[aoc_generator(day9)]
fn input_gen(input: &[u8]) -> Result<Readings> {
    let line = list(integer(), sym(b' '));
    let parser = list(line, sym(b'\n')) - end();
    Ok(Readings {
        lines: parser.parse(input)?,
    })
}

fn get_tree(readings: &[i64]) -> Vec<Vec<i64>> {
    let mut list = vec![readings.to_owned()];
    while list.last().unwrap().iter().any(|i| *i != 0) {
        let last = list.last().unwrap();
        list.push(
            last.iter()
                .zip(last.iter().skip(1))
                .map(|(i1, i2)| i2 - i1)
                .collect_vec(),
        );
    }
    list
}
fn extrapolate_last(readings: &[i64]) -> i64 {
    get_tree(readings)
        .into_iter()
        .map(|r| *r.last().unwrap())
        .sum()
}
fn extrapolate_first(readings: &[i64]) -> i64 {
    extrapolate_last(&readings.iter().copied().rev().collect_vec())
}

#[aoc(day9, part1)]
fn part1(input: &Readings) -> i64 {
    input.lines.iter().map(|l| extrapolate_last(l)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &Readings) -> i64 {
    input.lines.iter().map(|l| extrapolate_first(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 114);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 2);
    }
}
