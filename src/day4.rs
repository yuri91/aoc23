use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use logos::Logos;
use std::collections::HashSet;
use std::collections::VecDeque;

#[enpow::enpow(Var, ExpectVar)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ :]+")]
enum Token {
    #[token("Card")]
    Card,
    #[token("|")]
    Separator,
    #[regex("[0-9]+", |lex| lex.slice().parse().ok(), priority=100)]
    Num(u32),
    #[token("\n")]
    Newline,
    Error,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    id: u32,
    points: u32,
}
impl Card {
    fn new(id: u32) -> Card {
        Card { id, points: 0 }
    }
}

#[aoc_generator(day4)]
fn input_gen(input: &str) -> Result<Vec<Card>> {
    let mut lex = Token::lexer(input)
        .map(|t| t.unwrap_or(Token::Error))
        .chain(Some(Token::Newline));
    let mut ret = Vec::new();
    while let Some(t) = lex.next() {
        t.card().ok_or_else(|| anyhow!("expected 'Card'"))?;
        let n = lex
            .next()
            .and_then(Token::num)
            .ok_or_else(|| anyhow!("Expected num"))?;
        let mut c = Card::new(n);
        let mut winning = HashSet::new();
        loop {
            match lex.next() {
                Some(Token::Num(n)) => {
                    winning.insert(n);
                }
                Some(Token::Separator) => {
                    break;
                }
                _ => {
                    bail!("unexpected token");
                }
            }
        }
        loop {
            match lex.next() {
                Some(Token::Num(n)) => {
                    c.points += winning.contains(&n) as u32;
                }
                Some(Token::Newline) => {
                    break;
                }
                _ => {
                    bail!("unexpected token");
                }
            }
        }
        ret.push(c);
    }
    Ok(ret)
}
#[aoc(day4, part1)]
fn part1(input: &[Card]) -> u32 {
    let mut sum = 0;
    for c in input {
        sum += if c.points == 0 {
            0
        } else {
            1 << (c.points - 1)
        };
    }
    sum
}

#[aoc(day4, part2, slow)]
fn part2_slow(input: &[Card]) -> u32 {
    let mut sum = 0;
    let mut queue: VecDeque<Card> = input.iter().copied().collect();
    while let Some(top) = queue.pop_front() {
        sum += 1;
        for i in 0..top.points {
            if let Some(c) = input.get((top.id + i) as usize) {
                assert_eq!(c.id, top.id + i + 1);
                queue.push_back(*c);
            }
        }
    }
    sum
}

#[aoc(day4, part2, fast)]
fn part2_fast(input: &[Card]) -> u32 {
    let mut sum = 0;
    let mut queue: VecDeque<(Card, u32)> = input.iter().copied().map(|c| (c, 1)).collect();
    while let Some((top, n)) = queue.pop_front() {
        sum += n;
        for i in 0..top.points {
            if let Some((_, ref mut k)) = queue.get_mut(i as usize) {
                *k += n;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 13);
    }
    #[test]
    fn part2_slow_example() {
        assert_eq!(part2_slow(&input_gen(EXAMPLE).unwrap()), 30);
    }
    #[test]
    fn part2_fast_example() {
        assert_eq!(part2_fast(&input_gen(EXAMPLE).unwrap()), 30);
    }
}
