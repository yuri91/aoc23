use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use logos::Logos;
use std::collections::HashMap;
use std::ops::Range;

#[enpow::enpow(Var, ExpectVar)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"([ :]+)|map")]
enum Token {
    #[token("seeds")]
    Seeds,
    #[regex("[A-z]+-to-[A-z]+", |lex| lex.slice().split_once("-to-").map(|(a,b)| (a.to_owned(), b.to_owned())))]
    Map((String, String)),
    #[regex("[0-9]+", |lex| lex.slice().parse().ok(), priority=100)]
    Num(u64),
    #[token("\n")]
    Newline,
    #[token("\n\n")]
    DoubleNewline,
    Error,
}

#[derive(Debug)]
struct Mapping {
    from: Range<u64>,
    to: Range<u64>,
}
#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    dir: HashMap<String, (String, Vec<Mapping>)>,
}

#[aoc_generator(day5)]
fn input_gen(input: &str) -> Result<Almanac> {
    let mut lex = Token::lexer(input)
        .map(|t| t.unwrap_or(Token::Error))
        .chain(Some(Token::DoubleNewline));
    lex.next()
        .and_then(Token::seeds)
        .ok_or_else(|| anyhow!("expected seeds"))?;
    let mut seeds = Vec::new();
    loop {
        match lex.next() {
            Some(Token::Num(n)) => {
                seeds.push(n);
            }
            Some(Token::DoubleNewline) => {
                break;
            }
            other => {
                bail!("unexpected token {:?}", other);
            }
        }
    }
    let mut almanac = Almanac {
        seeds,
        dir: HashMap::new(),
    };
    'maps: while let Some(t) = lex.next() {
        let (orig, dest) = t.map().ok_or_else(|| anyhow!("expected map"))?;
        lex.next()
            .and_then(Token::newline)
            .ok_or_else(|| anyhow!("expected newline"))?;
        let mut mappings = Vec::new();
        loop {
            let to_start = lex
                .next()
                .and_then(Token::num)
                .ok_or_else(|| anyhow!("expected range to"))?;
            let from_start = lex
                .next()
                .and_then(Token::num)
                .ok_or_else(|| anyhow!("expected range from"))?;
            let len = lex
                .next()
                .and_then(Token::num)
                .ok_or_else(|| anyhow!("expected range len"))?;
            mappings.push(Mapping {
                from: from_start..(from_start + len),
                to: to_start..(to_start + len),
            });
            match lex.next() {
                Some(Token::Newline) => {
                    continue;
                }
                Some(Token::DoubleNewline) => {
                    almanac.dir.insert(orig, (dest, mappings));
                    continue 'maps;
                }
                _ => {
                    bail!("unexpected token");
                }
            }
        }
    }
    Ok(almanac)
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> Result<u64> {
    let mut min_loc = u64::MAX;
    for s in &input.seeds {
        let mut cur = "seed";
        let mut v = *s;
        while cur != "location" {
            let (next, mappings) = input.dir.get(cur).ok_or_else(|| anyhow!("missing key"))?;
            cur = next;
            for m in mappings {
                if m.from.contains(&v) {
                    v = m.to.start + (v - m.from.start);
                    break;
                }
            }
        }
        min_loc = min_loc.min(v);
    }
    Ok(min_loc)
}

fn map_range(s: Range<u64>, mappings: &[Mapping]) -> Vec<Range<u64>> {
    let mut map = Vec::new();
    let mut cur = s.start;
    while cur != s.end {
        'search: {
            for m in mappings {
                if m.from.contains(&cur) {
                    let cur_mapped = m.to.start + (cur - m.from.start);
                    let cur_mapped_end = cur_mapped + (s.end - cur).min(m.from.end - cur);
                    map.push(cur_mapped..cur_mapped_end);
                    cur += cur_mapped_end - cur_mapped;
                    break 'search;
                }
            }
            for m in mappings {
                if (cur..s.end).contains(&m.from.start) {
                    map.push(cur..m.from.start);
                    cur = m.from.start;
                    break 'search;
                }
            }
            map.push(s.clone());
            cur = s.end;
        }
    }
    map
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> Result<u64> {
    let mut seed_ranges = Vec::new();
    for i in (0..input.seeds.len()).step_by(2) {
        seed_ranges.push(input.seeds[i]..(input.seeds[i] + input.seeds[i + 1]));
    }
    seed_ranges.sort_by_key(|s| s.start);
    let mut cur = "seed";
    while cur != "location" {
        let mut next_ranges = Vec::new();
        let (next, mappings) = input.dir.get(cur).ok_or_else(|| anyhow!("missing key"))?;
        for s in seed_ranges {
            let mut mapped = map_range(s, mappings);
            next_ranges.append(&mut mapped);
        }
        cur = next;
        seed_ranges = next_ranges;
    }
    seed_ranges.sort_by_key(|s| s.start);
    Ok(seed_ranges.first().unwrap().start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()).unwrap(), 35);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()).unwrap(), 46);
    }
}
