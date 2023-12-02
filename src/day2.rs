use aoc_runner_derive::{aoc_generator, aoc};
use anyhow::{anyhow, bail, Result};
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ :]+")]
enum Token {
    #[token("Game")]
    Game,
    #[token("red")]
    Red,
    #[token("green")]
    Green,
    #[token("blue")]
    Blue,
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Num(u32),
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("\n")]
    Newline,
}

#[derive(Default, Debug)]
struct Round {
    blue: u32,
    green: u32,
    red: u32,
}
#[derive(Default, Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}
impl Game {
    fn new(id: u32) -> Game {
        Game { id, rounds: vec![] }
    }
}

trait LexerExt {
    fn get_next_opt(&mut self) -> Result<Option<Token>>;
    fn get_next(&mut self) -> Result<Token>;
    fn expect(&mut self, t: Token) -> Result<()>;
    fn get_num(&mut self) -> Result<u32>;
}
impl LexerExt for logos::Lexer<'_, Token> {
    fn get_next_opt(&mut self) -> Result<Option<Token>> {
        Ok(match self.next() {
            None => None,
            Some(Err(_)) => {
                bail!("lexing error. slice: {}", self.slice());
            },
            Some(Ok(t)) => Some(t)
        })
    }
    fn get_next(&mut self) -> Result<Token> {
        self.get_next_opt()?.ok_or_else(|| anyhow!("unexpected lexer end"))
    }
    fn expect(&mut self, t: Token) -> Result<()> {
        if self.get_next()? != t {
            bail!("unexpected token");
        }
        Ok(())
    }
    fn get_num(&mut self) -> Result<u32> {
        if let Token::Num(n) = self.get_next()? {
            Ok(n)
        } else {
            bail!("unexpected token");
        }
    }
}

#[aoc_generator(day2)]
fn input_gen(input: &str) -> Result<Vec<Game>> {
    let mut games = Vec::new();
    let mut lex = Token::lexer(input);
    'games: while let Some(t) = lex.get_next_opt()? {
        if t != Token::Game {
            bail!("expected Game");
        }
        let mut game = Game::new(lex.get_num()?);
        'game: loop {
            let mut round =  Round::default();
            loop {
                let n = lex.get_num()?;
                match lex.get_next()? {
                    Token::Red => {
                        round.red += n;
                    },
                    Token::Green => {
                        round.green += n;
                    },
                    Token::Blue => {
                        round.blue += n;
                    },
                    _ => {
                        bail!("unexpected token");
                    },
                }
                match lex.get_next_opt()? {
                    Some(Token::Comma) => {
                        continue;
                    },
                    Some(Token::Semicolon) => {
                        break;
                    },
                    Some(Token::Newline) => {
                        game.rounds.push(round);
                        break 'game;
                    },
                    None => {
                        game.rounds.push(round);
                        games.push(game);
                        break 'games;
                    }
                    _ => {
                        bail!("unexpected token");
                    },
                }
            }
            game.rounds.push(round);
        }
        games.push(game);
    }
    Ok(games)
}
#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    const TOT_RED: u32 = 12;
    const TOT_GREEN: u32 = 13;
    const TOT_BLUE: u32 = 14;
    input.iter().filter(|g| {
        for r in &g.rounds {
            if r.red > TOT_RED {
                return false;
            }
            if r.green > TOT_GREEN {
                return false;
            }
            if r.blue > TOT_BLUE {
                return false;
            }
        }
        true
    }).map(|g| {
        g.id
    })
    .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input.iter().map(|g| {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for r in &g.rounds {
            min_red = min_red.max(r.red);
            min_green = min_green.max(r.green);
            min_blue = min_blue.max(r.blue);
        }
        min_green * min_blue * min_red
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str =
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 8);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 2286);
    }
}
