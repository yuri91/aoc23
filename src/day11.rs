use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pom::parser::*;

fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'#').map(|_| Tile::Galaxy) | sym(b'.').map(|_| Tile::Empty)
}
fn row<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}
fn map<'a>() -> Parser<'a, u8, Vec<Vec<Tile>>> {
    row().repeat(1..)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Galaxy,
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<Tile>>,
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

fn expand_rows(map: &Map, factor: i64) -> Vec<i64> {
    map.data
        .iter()
        .map(|row| {
            if row.iter().all(|t| *t == Tile::Empty) {
                factor
            } else {
                1
            }
        })
        .collect_vec()
}
fn expand_cols(map: &Map, factor: i64) -> Vec<i64> {
    let mut cols = Vec::new();
    for x in 0..map.data[0].len() {
        let mut all_empty = true;
        for y in 0..map.data.len() {
            if map.data[y][x] != Tile::Empty {
                all_empty = false;
                break;
            }
        }
        cols.push(if all_empty { factor } else { 1 });
    }
    cols
}

fn get_galaxies(map: &Map, expansion_factor: i64) -> Vec<Pos> {
    let exp_rows = expand_rows(map, expansion_factor);
    let exp_cols = expand_cols(map, expansion_factor);
    let mut ret = Vec::new();
    let mut actual_y = 0;
    for (y, row) in map.data.iter().enumerate() {
        let mut actual_x = 0;
        for (x, t) in row.iter().enumerate() {
            if *t == Tile::Galaxy {
                ret.push(Pos {
                    x: actual_x,
                    y: actual_y,
                });
            }
            actual_x += exp_cols[x];
        }
        actual_y += exp_rows[y];
    }
    ret
}

#[aoc_generator(day11)]
fn input_gen(input: &[u8]) -> Result<Map> {
    Ok(Map {
        data: map().parse(input)?,
    })
}

fn distances_with_factor(input: &Map, factor: i64) -> i64 {
    let mut ret = 0;
    let gs = get_galaxies(input, factor);
    for p in gs.into_iter().combinations(2) {
        let [p1, p2] = p[..] else { unreachable!() };
        let dist = (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
        ret += dist;
    }
    ret
}

#[aoc(day11, part1)]
fn part1(input: &Map) -> i64 {
    distances_with_factor(input, 2)
}

#[aoc(day11, part2)]
fn part2(input: &Map) -> i64 {
    distances_with_factor(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 374);
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            distances_with_factor(&input_gen(EXAMPLE).unwrap(), 100),
            8410
        );
    }
}
