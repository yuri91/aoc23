use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
pub fn input_digits(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            let d1 = l
                .chars()
                .find(|c| c.is_ascii_digit())
                .ok_or_else(|| anyhow!("no digits in line"))?;
            let d2 = l
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .ok_or_else(|| anyhow!("no digits in line"))?;
            Ok(10 * d1.to_digit(10).unwrap() + d2.to_digit(10).unwrap())
        })
        .collect::<Result<_>>()
}

#[aoc_generator(day1, part2)]
pub fn input_digits_words(input: &str) -> Result<Vec<u32>> {
    let matchers: &[(&str, u32)] = &[
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let first_prefix = |s: &str| -> u32 {
        let mut iv = 0;
        let mut mini = std::usize::MAX;
        for (m, v) in matchers {
            let i = s.find(m).unwrap_or(std::usize::MAX);
            if i < mini {
                mini = i;
                iv = *v;
            }
        }
        iv
    };
    let last_prefix = |s: &str| -> u32 {
        let mut iv = 0;
        let mut maxi = 0;
        for (m, v) in matchers {
            let i = s.rfind(m).map(|i| i + m.len()).unwrap_or(0);
            if i >= maxi {
                maxi = i;
                iv = *v;
            }
        }
        iv
    };
    Ok(input
        .lines()
        .map(|l| {
            let d1 = first_prefix(l);
            let d2 = last_prefix(l);
            d1 * 10 + d2
        })
        .collect())
}

#[aoc(day1, part1)]
#[aoc(day1, part2)]
pub fn day1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_simple() {
        assert_eq!(day1(&input_digits_words("1").unwrap()), 11);
    }
    #[test]
    fn part2_simple() {
        assert_eq!(day1(&input_digits_words("one").unwrap()), 11);
    }
    #[test]
    fn part2_example() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(
            &input_digits_words(input).unwrap(),
            &[29, 83, 13, 24, 42, 14, 76]
        );
        assert_eq!(day1(&input_digits_words(input).unwrap()), 281);
    }
}
