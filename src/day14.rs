use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::{s, Array2, Axis};
use pom::parser::*;

fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'O').map(|_| Tile::Round) | sym(b'#').map(|_| Tile::Cube) | sym(b'.').map(|_| Tile::Empty)
}
fn row<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}
fn map<'a>() -> Parser<'a, u8, Vec<Vec<Tile>>> {
    row().repeat(1..)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Round,
    Cube,
}

#[aoc_generator(day14)]
fn input_gen(input: &[u8]) -> Result<Array2<Tile>> {
    let data = map().parse(input)?;
    let shape = (data.len(), data[0].len());
    let flat = data.into_iter().flatten().collect_vec();
    Ok(Array2::from_shape_vec(shape, flat)?)
}

fn tilt(map: &mut Array2<Tile>, axis: Axis, dir: i32) {
    for mut lane in map.axis_iter_mut(axis) {
        let mut slice = lane.slice_mut(s![..; dir]);
        for i in 1..slice.len() {
            if slice[i] == Tile::Round {
                let mut j = i;
                loop {
                    if slice[j - 1] != Tile::Empty {
                        break;
                    }
                    j -= 1;
                    if j == 0 {
                        break;
                    }
                }
                slice.swap(i, j);
            }
        }
    }
}
fn cycle(map: &mut Array2<Tile>) {
    tilt(map, Axis(1), 1);
    tilt(map, Axis(0), 1);
    tilt(map, Axis(1), -1);
    tilt(map, Axis(0), -1);
}

fn load(m: &Array2<Tile>) -> i64 {
    let mut ret = 0;
    for (i, r) in m.rows().into_iter().enumerate() {
        let n = r.map(|&t| if t == Tile::Round { 1 } else { 0 }).sum();
        ret += ((m.shape()[0] - i) * n) as i64;
    }
    ret
}

#[aoc(day14, part1)]
fn part1(input: &Array2<Tile>) -> i64 {
    let mut m = input.clone();
    tilt(&mut m, Axis(1), 1);
    load(&m)
}

#[aoc(day14, part2)]
fn part2(input: &Array2<Tile>) -> i64 {
    let mut m = input.clone();
    let mut hist = Vec::new();
    let period;
    let periodic_from;
    loop {
        hist.push(m.clone());
        cycle(&mut m);
        if let Some(idx) = hist.iter().position(|h| h == m) {
            periodic_from = idx;
            period = hist.len() - idx;
            break;
        }
    }
    let n = 1000000000;
    let final_m = &hist[periodic_from + (n - periodic_from) % period];
    load(final_m)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 136);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 64);
    }
}
