use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use logos::Logos;
use std::convert::TryFrom;

#[enpow::enpow(Var, ExpectVar)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"")]
enum Token {
    #[regex("[2-9AKQJT]", |lex| lex.slice().chars().next(), priority=101)]
    Card(char),
    #[regex(" [0-9]+", |lex| lex.slice()[1..].parse().ok(), priority=100)]
    Num(i64),
    #[token("\n")]
    Newline,
    Error,
}

#[enpow::enpow(Var, IsVar)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}
impl TryFrom<char> for Card {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self> {
        use Card::*;
        Ok(match value {
            '2' => N2,
            '3' => N3,
            '4' => N4,
            '5' => N5,
            '6' => N6,
            '7' => N7,
            '8' => N8,
            '9' => N9,
            'T' => T,
            'J' => J,
            'Q' => Q,
            'K' => K,
            'A' => A,
            _ => {
                bail!("unexpected card");
            }
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn hand_kind(hand: [Card; 5]) -> HandKind {
    use HandKind::*;
    let mut h = hand
        .into_iter()
        .sorted()
        .dedup_with_count()
        .sorted_by_key(|(count, _)| *count)
        .rev()
        .collect_vec();
    let jokers = h
        .iter()
        .find_map(|(n, c)| c.joker().map(|_| *n))
        .unwrap_or(0);
    h.retain(|(_, c)| !c.is_joker());
    if let Some((ref mut n, _)) = h.first_mut() {
        *n += jokers;
    } else {
        h.push((5, Card::Joker));
    }
    match h.as_slice() {
        &[(5, _)] => Five,
        &[(4, _), ..] => Four,
        &[(3, _), (2, _)] => Full,
        &[(3, _), ..] => Three,
        &[(2, _), (2, _), ..] => TwoPair,
        &[(2, _), ..] => OnePair,
        _ => High,
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match hand_kind(self.cards).cmp(&hand_kind(other.cards)) {
            Less => Less,
            Greater => Greater,
            Equal => self.cards.cmp(&other.cards),
        }
    }
}
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day7)]
fn input_gen(input: &str) -> Result<Vec<Hand>> {
    let mut lex = Token::lexer(input)
        .map(|t| t.unwrap_or(Token::Error))
        .chain(Some(Token::Newline))
        .peekable();

    let mut hands = Vec::new();
    while lex.peek().is_some() {
        let mut hand = Hand {
            bid: 0,
            cards: [Card::N2; 5],
        };
        for i in 0..5 {
            match lex.next() {
                Some(Token::Card(c)) => {
                    hand.cards[i] = c.try_into()?;
                }
                t => {
                    bail!("unexpected token: '{t:?}'");
                }
            }
        }
        hand.bid = lex
            .next()
            .and_then(Token::num)
            .ok_or_else(|| anyhow!("expected bid"))?;
        hands.push(hand);
        lex.next()
            .and_then(Token::newline)
            .ok_or_else(|| anyhow!("expected newline"))?;
    }
    Ok(hands)
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> i64 {
    input
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, h)| (i as i64 + 1) * h.bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Hand]) -> i64 {
    let input = input
        .iter()
        .map(|h| {
            let mut h = *h;
            for c in &mut h.cards {
                if *c == Card::J {
                    *c = Card::Joker;
                }
            }
            h
        })
        .collect_vec();
    part1(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 6440);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 5905);
    }
}
