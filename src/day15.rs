use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use pom::parser::*;
use std::num::Wrapping;

fn step<'a>() -> Parser<'a, u8, Step> {
    let digit = one_of(b"123456789").map(|d| d - b'0');
    let eq_digit = sym(b'=') * digit;
    let minus = sym(b'-').map(|_| 10u8);
    (one_of(b"abcdefghijklmnopqrstuvwxyz").repeat(1..) + (minus | eq_digit)).map(|(l, n)| match n {
        10 => Step::Remove(l),
        s => Step::Assign(l, s),
    })
}
fn seq<'a>() -> Parser<'a, u8, Vec<Step>> {
    list(step(), sym(b',')) - end()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Step {
    Assign(Vec<u8>, u8),
    Remove(Vec<u8>),
}

trait Hash {
    fn hash(&self, hasher: &mut Wrapping<u8>);
}
impl Hash for Step {
    fn hash(&self, hasher: &mut Wrapping<u8>) {
        match self {
            Step::Assign(l, n) => {
                l.hash(hasher);
                b'='.hash(hasher);
                (b'0' + n).hash(hasher);
            }
            Step::Remove(l) => {
                l.hash(hasher);
                b'-'.hash(hasher);
            }
        }
    }
}
impl Hash for Vec<u8> {
    fn hash(&self, hasher: &mut Wrapping<u8>) {
        for c in self {
            c.hash(hasher);
        }
    }
}
impl Hash for u8 {
    fn hash(&self, hasher: &mut Wrapping<u8>) {
        *hasher += *self;
        *hasher *= 17;
    }
}

#[aoc_generator(day15)]
fn input_gen(input: &[u8]) -> Result<Vec<Step>> {
    let seq = seq().parse(input)?;
    Ok(seq)
}

fn hash<T: Hash>(t: &T) -> usize {
    let mut hasher = Wrapping(0);
    t.hash(&mut hasher);
    hasher.0 as usize
}

#[aoc(day15, part1)]
fn part1(input: &[Step]) -> usize {
    input.iter().map(hash).sum()
}

#[aoc(day15, part2)]
fn part2(input: &[Step]) -> usize {
    let mut buckets: [Vec<(Vec<u8>, u8)>; 256] = std::array::from_fn(|_| Vec::new());
    for step in input {
        match step {
            Step::Assign(label, fl) => {
                let idx = hash(label);
                let bucket = &mut buckets[idx];
                if let Some((_, ref mut n)) = bucket.iter_mut().find(|(l, _)| l == label) {
                    *n = *fl;
                } else {
                    bucket.push((label.to_owned(), *fl));
                }
            }
            Step::Remove(label) => {
                let idx = hash(label);
                let bucket = &mut buckets[idx];
                *bucket = bucket.iter().filter(|(l, _)| l != label).cloned().collect();
            }
        }
    }
    let mut ret = 0;
    for (bidx, bucket) in buckets.iter().enumerate() {
        for (lidx, (_, focus)) in bucket.iter().enumerate() {
            ret += (bidx + 1) * (lidx + 1) * (*focus as usize);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 1320);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 145);
    }
}
