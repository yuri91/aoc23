use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use pom::parser::*;
use std::collections::HashMap;

fn unsigned<'a>() -> Parser<'a, u8, i64> {
    let n = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    n.collect()
        .convert(std::str::from_utf8)
        .convert(|s| s.parse())
}
fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'.').map(|_| Tile::Ok) | sym(b'#').map(|_| Tile::Broken) | sym(b'?').map(|_| Tile::Unknonw)
}
fn tiles<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..)
}
fn groups<'a>() -> Parser<'a, u8, Vec<i64>> {
    list(unsigned(), sym(b',')).name("groups")
}

fn row<'a>() -> Parser<'a, u8, Row> {
    (tiles() - sym(b' ') + groups() - (sym(b'\n').discard() | end())).map(|(t, g)| Row {
        tiles: t,
        groups: g,
    })
}
fn map<'a>() -> Parser<'a, u8, Map> {
    row().repeat(1..).map(|data| Map { data })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Ok,
    Broken,
    Unknonw,
}

#[derive(Debug, Clone)]
struct Row {
    tiles: Vec<Tile>,
    groups: Vec<i64>,
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Row>,
}

#[aoc_generator(day12)]
fn input_gen(input: &[u8]) -> Result<Map> {
    Ok(map().parse(input)?)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Search {
    tiles: Vec<Tile>,
    groups: Vec<i64>,
    cur_streak: usize,
}

fn search(mut s: Search, cache: &mut HashMap<Search, i64>) -> i64 {
    if let Some(&e) = cache.get(&s) {
        return e;
    }
    let key = s.clone();
    let mut ret = 0;
    let mut group = s.groups.first().copied().unwrap_or(0) as usize;
    for (i, &(mut t)) in s.tiles.iter().enumerate() {
        if t == Tile::Unknonw {
            let mut new_search = Search {
                tiles: s.tiles[i..].to_owned(),
                groups: s.groups.clone(),
                cur_streak: s.cur_streak,
            };
            new_search.tiles[0] = Tile::Ok;
            ret += search(new_search, cache);
            t = Tile::Broken;
        }
        match t {
            Tile::Broken => {
                if s.groups.is_empty() {
                    cache.insert(key, ret);
                    return ret;
                }
                s.cur_streak += 1;
                if s.cur_streak > group {
                    cache.insert(key, ret);
                    return ret;
                }
            }
            Tile::Ok => {
                if s.cur_streak != 0 {
                    if s.cur_streak == group {
                        s.cur_streak = 0;
                        s.groups.remove(0);
                        group = s.groups.first().copied().unwrap_or(0) as usize;
                    } else {
                        cache.insert(key, ret);
                        return ret;
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    if s.groups.is_empty() {
        ret += 1;
    }
    cache.insert(key, ret);
    ret
}

fn do_search(input: Map) -> i64 {
    let mut ret = 0;
    let mut cache = HashMap::new();
    for row in &input.data {
        let Row { mut tiles, groups } = row.clone();
        tiles.push(Tile::Ok);
        let s = Search {
            tiles,
            groups,
            cur_streak: 0,
        };
        ret += search(s, &mut cache);
    }
    ret
}
#[aoc(day12, part1)]
fn part1(input: &Map) -> i64 {
    do_search(input.clone())
}

#[aoc(day12, part2)]
fn part2(input: &Map) -> i64 {
    let mut m = input.clone();
    for row in &mut m.data {
        let new_tiles = std::iter::once(Tile::Unknonw)
            .chain(row.tiles.clone().into_iter())
            .cycle()
            .take((row.tiles.len() + 1) * 4);
        let new_groups = row
            .groups
            .clone()
            .into_iter()
            .cycle()
            .take(row.groups.len() * 4);
        row.tiles.extend(new_tiles);
        row.groups.extend(new_groups);
    }
    do_search(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 21);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 525152);
    }
}
