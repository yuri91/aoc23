use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use logos::Logos;

use std::collections::HashMap;

#[enpow::enpow(Var, ExpectVar, IsVar)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r" +")]
enum Token {
    #[token("=")]
    Eq,
    #[token("(")]
    LPar,
    #[token(")")]
    RPar,
    #[token(",")]
    Comma,
    #[regex("[A-Z1-9]+", |lex| lex.slice().to_string())]
    Seq(String),
    #[token("\n")]
    Newline,
    #[token("\n\n")]
    DoubleNewline,
    Error(String),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map {
    nodes: HashMap<String, (String, String)>,
    directions: Vec<Direction>,
}

#[aoc_generator(day8)]
fn input_gen(input: &str) -> Result<Map> {
    let mut lex = Token::lexer(input)
        .spanned()
        .map(|(t, s)| t.unwrap_or(Token::Error(input[s].to_owned())))
        .chain(Some(Token::Newline))
        .peekable();

    let mut directions = Vec::new();
    while lex.peek().map(|t| !t.is_double_newline()) == Some(true) {
        match lex.next() {
            Some(Token::Seq(s)) => {
                for d in s.chars() {
                    match d {
                        'L' => {
                            directions.push(Direction::Left);
                        }
                        'R' => {
                            directions.push(Direction::Right);
                        }
                        _ => {
                            bail!("wrong direction");
                        }
                    }
                }
            }
            t => {
                bail!("unexpected token {t:?}");
            }
        }
    }
    lex.next().and_then(Token::double_newline).unwrap();
    let mut nodes = HashMap::new();
    while lex.peek().map(|t| !t.is_double_newline()) == Some(true) {
        let from = lex
            .next()
            .and_then(Token::seq)
            .ok_or_else(|| anyhow!("expected place"))?;
        lex.next()
            .and_then(Token::eq)
            .ok_or_else(|| anyhow!("expected ="))?;
        lex.next()
            .and_then(Token::lpar)
            .ok_or_else(|| anyhow!("expected ("))?;
        let left = lex
            .next()
            .and_then(Token::seq)
            .ok_or_else(|| anyhow!("expected place"))?;
        lex.next()
            .and_then(Token::comma)
            .ok_or_else(|| anyhow!("expected ,"))?;
        let right = lex
            .next()
            .and_then(Token::seq)
            .ok_or_else(|| anyhow!("expected place"))?;
        lex.next()
            .and_then(Token::rpar)
            .ok_or_else(|| anyhow!("expected )"))?;
        lex.next()
            .and_then(Token::newline)
            .ok_or_else(|| anyhow!("expected newline"))?;
        nodes.insert(from, (left, right));
    }

    Ok(Map { directions, nodes })
}

#[aoc(day8, part1)]
fn part1(input: &Map) -> Result<i64> {
    let mut cur = "AAA";
    for (i, d) in input.directions.iter().cycle().enumerate() {
        let (left, right) = input
            .nodes
            .get(cur)
            .ok_or_else(|| anyhow!("no next node"))?;
        cur = match d {
            Direction::Left => left,
            Direction::Right => right,
        };
        if cur == "ZZZ" {
            return Ok((i + 1) as i64);
        }
    }
    unreachable!();
}

#[aoc(day8, part2)]
fn part2(input: &Map) -> Result<i64> {
    let mut curs = input
        .nodes
        .iter()
        .filter(|(k, _)| k[2..].starts_with('A'))
        .map(|(k, _)| k)
        .sorted()
        .collect_vec();
    let mut times_to_z = vec![0; curs.len()];
    let mut full_periods = times_to_z.len();
    'main: for (i, d) in input.directions.iter().cycle().enumerate() {
        for (cur, time_to_z) in curs.iter_mut().zip(times_to_z.iter_mut()) {
            let (left, right) = input
                .nodes
                .get(cur.as_str())
                .ok_or_else(|| anyhow!("no next node"))?;
            *cur = match d {
                Direction::Left => left,
                Direction::Right => right,
            };
            if cur[2..].starts_with('Z') {
                *time_to_z = (i + 1) as i64;
                full_periods -= 1;
                if full_periods == 0 {
                    break 'main;
                }
            }
        }
    }
    Ok(times_to_z
        .into_iter()
        .fold(1, |res, t| num::integer::lcm(t, res)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE1).unwrap()).unwrap(), 6);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE2).unwrap()).unwrap(), 6);
    }
}
