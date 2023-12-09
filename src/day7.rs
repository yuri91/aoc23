use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use logos::Logos;

#[enpow::enpow(Var, ExpectVar)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r" ")]
enum Token {
    #[regex(" [0-9]+", |lex| lex.slice().parse().ok(), priority=100)]
    Num(i64),
    #[regex("[2-9AKQJT]")]
    Card(char),
    #[token("\n")]
    Newline,
    Error,
}

#[derive(Copy, Clone, Debug)]
enum Card {
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
#[derive(Copy, Clone, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

fn parse_card(c: char) -> Result<Card> {
    use Card::*;
    Ok(match c {
        '2' => {
            N2
        }
        '3' => {
            N3
        }
        '4' => {
            N4
        }
        '5' => {
            N5
        }
        '6' => {
            N6
        }
        '7' => {
            N7
        }
        '8' => {
            N8
        }
        '9' => {
            N9
        }
        'T' => {
            T
        }
        'J' => {
            J
        }
        'Q' => {
            Q
        }
        'K' => {
            K
        }
        'A' => {
            A
        }
        _ => {
            bail!("unexpected card");
        }
    })
}

#[aoc_generator(day7)]
fn input_gen(input: &str) -> Result<Vec<Hand>> {
    let mut lex = Token::lexer(input)
        .map(|t| t.unwrap_or(Token::Error))
        .chain(Some(Token::Newline));
    
    let mut hands = Vec::new();
    loop {
        let mut hand = Hand {
            bid: 0,
            cards: [Card::N2; 5],
        };
        for i in 0..5 {
            match lex.next() {
                Some(Token::Card(c)) => {
                    hand.cards[i] = parse_card(c)?;
                },
                Some(Token::Newline) => {
                    anyhow::ensure!(i == 0, "expected more cards");
                },
                _ => {
                    bail!("unexpected token");
                }
            }
        }
        hand.bid = lex.next().and_then(Token::num).ok_or_else(|| anyhow!("expected bid"))?;
        hands.push(hand);
    }
    Ok(hands)
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> Result<i64> {
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()).unwrap(), 288);
    }
    #[test]
    fn part2_example() {
        //assert_eq!(part2(&input_gen(EXAMPLE).unwrap()).unwrap(), 71503);
    }
}
