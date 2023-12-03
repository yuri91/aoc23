use anyhow::{bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use logos::Logos;

#[enpow::enpow(All)]
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\.+")]
enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse().ok(), priority=100)]
    Num(u32),
    #[regex(".", |lex| lex.slice().chars().next(), priority=1)]
    Symbol(char),
    #[token("\n")]
    Newline,
    Error,
}

#[derive(Debug, Default)]
struct Part {
    id: u32,
    x0: i32,
    x1: i32,
    y: i32,
}
#[derive(Debug, Default)]
struct Symbol {
    c: char,
    x: i32,
    y: i32,
}
#[derive(Debug, Default)]
struct Matrix {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

#[aoc_generator(day3)]
fn input_gen(input: &str) -> Result<Matrix> {
    let mut lex = Token::lexer(input)
        .spanned()
        .map(|(t, s)| (t.unwrap_or_else(|_| Token::Error), s))
        .chain(Some((Token::Newline, 0usize..0usize)));
    let mut mat = Matrix::default();
    let mut cur_y = 0;
    let mut x_len = 0;
    while let Some((t, s)) = lex.next() {
        match t {
            Token::Num(id) => {
                mat.parts.push(Part {
                    id,
                    x0: (s.start - x_len * cur_y) as i32,
                    x1: (s.end - 1 - x_len * cur_y) as i32,
                    y: cur_y as i32,
                });
            }
            Token::Symbol(c) => {
                mat.symbols.push(Symbol {
                    c,
                    x: (s.start - x_len * cur_y) as i32,
                    y: cur_y as i32,
                });
            }
            Token::Newline => {
                if x_len == 0 {
                    x_len = s.start + 1;
                }
                cur_y += 1;
            }
            Token::Error => {
                bail!("lexer error");
            }
        }
    }
    Ok(mat)
}
fn is_adjacent(p: &Part, s: &Symbol) -> bool {
    if s.y > p.y + 1 || s.y < p.y - 1 {
        false
    } else if s.x < p.x0 - 1 || s.x > p.x1 + 1 {
        false
    } else {
        true
    }
}

#[aoc(day3, part1)]
fn part1(input: &Matrix) -> u32 {
    let mut sum = 0;
    for p in &input.parts {
        for s in &input.symbols {
            if is_adjacent(p, s) {
                sum += p.id;
                break;
            }
        }
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &Matrix) -> u32 {
    let mut sum = 0;
    for s in input.symbols.iter().filter(|s| s.c == '*') {
        let mut nparts = 0;
        let mut cur_ratio = 1;
        for p in &input.parts {
            if is_adjacent(p, s) {
                nparts += 1;
                cur_ratio *= p.id;
                if nparts == 2 {
                    sum += cur_ratio;
                    break;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 4361);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 467835);
    }
}
