use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use logos::Logos;

#[enpow::enpow(Var, ExpectVar)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ ]+")]
enum Token {
    #[token("Time:")]
    Time,
    #[token("Distance:")]
    Distance,
    #[regex("[0-9]+", |lex| lex.slice().parse().ok(), priority=100)]
    Num(i64),
    #[token("\n")]
    Newline,
    Error,
}

#[derive(Debug)]
struct Doc {
    times: Vec<i64>,
    distances: Vec<i64>,
}

fn parse_vec_num(lex: &mut impl Iterator<Item=Token>) -> Result<Vec<i64>> {
    let mut ret = Vec::new();
    loop {
        match lex.next() {
            Some(Token::Num(n)) => {
                ret.push(n);
            },
            Some(Token::Newline) => {
                break;
            },
            _ => {
                bail!("unexpected token");
            }
        }
    }
    Ok(ret)
}
#[aoc_generator(day6)]
fn input_gen(input: &str) -> Result<Doc> {
    let mut lex = Token::lexer(input)
        .map(|t| t.unwrap_or(Token::Error))
        .chain(Some(Token::Newline));
    lex.next().map(Token::time).ok_or_else(|| anyhow!("expected time"))?;
    let times = parse_vec_num(&mut lex)?;
    lex.next().map(Token::distance).ok_or_else(|| anyhow!("expected distance"))?;
    let distances = parse_vec_num(&mut lex)?;
    Ok( Doc {
        times,
        distances,
    })
}

fn input_adapt_part2(input: &Doc) -> Doc {
    let mut time = 0;
    let mut cur_digits = 0;
    for t in input.times.iter().rev() {
        let digits = (*t as f64).log10().ceil() as i64;
        time += (10i64).pow(cur_digits as u32)*t;
        cur_digits += digits;
    }
    let mut distance = 0;
    let mut cur_digits = 0;
    for t in input.distances.iter().rev() {
        let digits = (*t as f64).log10().ceil() as i64;
        distance += (10i64).pow(cur_digits as u32)*t;
        cur_digits += digits;
    }
    Doc {
        times: vec![time],
        distances: vec![distance],
    }
}

fn viable_count(t: i64, d: i64) -> i64 {
    let mut ret = 0;
    for i in 1..(t-1) {
        let my_d = i*(t-i);
        if my_d > d {
            ret +=1;
        }
    }
    ret
}
#[aoc(day6, part1)]
fn part1(input: &Doc) -> Result<i64> {
    let mut ret = 1;
    for (d, t) in input.distances.iter().zip(input.times.iter()) {
        ret *= viable_count(*t, *d);
    }
    Ok(ret)
}
#[aoc(day6, part2)]
fn part2(input: &Doc) -> Result<i64> {
    let input = input_adapt_part2(input);
    part1(&input)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
r#"Time:      7  15   30
Distance:  9  40  200"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()).unwrap(), 288);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()).unwrap(), 71503);
    }
}
