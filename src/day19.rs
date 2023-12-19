use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use pom::parser::*;
use std::collections::HashMap;

fn ident<'a>() -> Parser<'a, u8, String> {
    one_of(b"abcdefghijklmnopqrstuvwxyz")
        .repeat(1..)
        .collect()
        .convert(std::str::from_utf8)
        .map(|s| s.to_owned())
}
fn categ<'a>() -> Parser<'a, u8, Category> {
    sym(b'x').map(|_| Category::X)
        | sym(b'm').map(|_| Category::M)
        | sym(b'a').map(|_| Category::A)
        | sym(b's').map(|_| Category::S)
}
fn op<'a>() -> Parser<'a, u8, Op> {
    sym(b'>').map(|_| Op::Gt) | sym(b'<').map(|_| Op::Lt)
}
fn dec<'a>() -> Parser<'a, u8, u64> {
    ((one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0'))
        .collect()
        .convert(std::str::from_utf8)
        .convert(|s| s.parse())
}

fn dest<'a>() -> Parser<'a, u8, Dest> {
    sym(b'A').map(|_| Dest::A) | sym(b'R').map(|_| Dest::R) | ident().map(Dest::Forward)
}
fn cond<'a>() -> Parser<'a, u8, Condition> {
    (categ() + op() + dec() - sym(b':') + dest()).map(|(((c, o), n), d)| Condition {
        categ: c,
        op: o,
        thresh: n,
        dest: d,
    })
}
fn workflow<'a>() -> Parser<'a, u8, Workflow> {
    (ident() - sym(b'{') + (cond() - sym(b',')).repeat(0..) + dest()
        - sym(b'}')
        - (sym(b'\n').discard() | end()))
    .map(|((name, conditions), default)| Workflow {
        name,
        conditions,
        default,
    })
}

fn part<'a>() -> Parser<'a, u8, Part> {
    (sym(b'{')
        * ((seq(b"x=") * dec() - sym(b','))
            + (seq(b"m=") * dec() - sym(b','))
            + (seq(b"a=") * dec() - sym(b','))
            + (seq(b"s=") * dec()))
        - sym(b'}')
        - (sym(b'\n').discard() | end()))
    .map(|(((x, m), a), s)| Part {
        scores: [x, m, a, s],
    })
}

fn system<'a>() -> Parser<'a, u8, System> {
    (workflow().repeat(1..) - sym(b'\n') + part().repeat(1..) - end()).map(|(w, p)| System {
        workflows: w,
        parts: p,
    })
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Lt,
    Gt,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Category {
    X = 0,
    M,
    A,
    S,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    scores: [u64; 4],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dest {
    A,
    R,
    Forward(String),
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Condition {
    categ: Category,
    op: Op,
    thresh: u64,
    dest: Dest,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    default: Dest,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct System {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

#[aoc_generator(day19)]
fn input_gen(input: &[u8]) -> Result<System> {
    Ok(system().parse(input)?)
}

fn match_cond(c: &Condition, p: &Part) -> bool {
    let idx = c.categ as usize;
    let v = p.scores[idx];
    match c.op {
        Op::Lt => v < c.thresh,
        Op::Gt => v > c.thresh,
    }
}

#[aoc(day19, part1)]
fn part1(input: &System) -> u64 {
    let table: HashMap<_, _> = input
        .workflows
        .iter()
        .map(|w| (w.name.clone(), w))
        .collect();
    input
        .parts
        .iter()
        .filter(|p| {
            let mut cur = "in".to_owned();
            'main: loop {
                let w = &table[&cur];
                for c in &w.conditions {
                    if match_cond(c, p) {
                        match c.dest {
                            Dest::A => {
                                return true;
                            }
                            Dest::R => {
                                return false;
                            }
                            Dest::Forward(ref next) => {
                                cur = next.clone();
                                continue 'main;
                            }
                        }
                    }
                }
                match w.default {
                    Dest::A => {
                        return true;
                    }
                    Dest::R => {
                        return false;
                    }
                    Dest::Forward(ref next) => {
                        cur = next.clone();
                        continue;
                    }
                }
            }
        })
        .map(|p| p.scores.iter().copied().sum::<u64>())
        .sum()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Constraints {
    min: u64,
    max: u64,
}
impl Default for Constraints {
    fn default() -> Constraints {
        Constraints { min: 1, max: 4000 }
    }
}
impl Constraints {
    fn add(&self, op: Op, thresh: u64) -> Option<Constraints> {
        let mut ret = *self;
        match op {
            Op::Gt => {
                ret.min = ret.min.max(thresh + 1);
            }
            Op::Lt => {
                ret.max = ret.max.min(thresh - 1);
            }
        }
        (ret.min < ret.max).then_some(ret)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PartProbe {
    scores: [Constraints; 4],
}
impl PartProbe {
    fn constrain(&self, cond: &Condition) -> (Option<PartProbe>, Option<PartProbe>) {
        let r1 = self.scores[cond.categ as usize]
            .add(cond.op, cond.thresh)
            .map(|c| {
                let mut ret = *self;
                ret.scores[cond.categ as usize] = c;
                ret
            });
        let (rev_op, rev_thresh) = if cond.op == Op::Gt {
            (Op::Lt, cond.thresh + 1)
        } else {
            (Op::Gt, cond.thresh - 1)
        };
        let r2 = self.scores[cond.categ as usize]
            .add(rev_op, rev_thresh)
            .map(|c| {
                let mut ret = *self;
                ret.scores[cond.categ as usize] = c;
                ret
            });
        (r1, r2)
    }
}

#[aoc(day19, part2)]
fn part2(input: &System) -> u64 {
    let table: HashMap<_, _> = input
        .workflows
        .iter()
        .map(|w| (w.name.clone(), w))
        .collect();
    let mut queue = vec![(PartProbe::default(), "in".to_owned())];
    let mut accepted = vec![];
    while let Some((mut p, cur)) = queue.pop() {
        let w = &table[&cur];
        for c in &w.conditions {
            let (is_match, is_not_match) = p.constrain(c);
            if let Some(newp) = is_match {
                match c.dest {
                    Dest::A => {
                        accepted.push(newp);
                    }
                    Dest::R => {}
                    Dest::Forward(ref next) => {
                        queue.push((newp, next.clone()));
                    }
                }
            }
            if let Some(newp) = is_not_match {
                p = newp;
            } else {
                break;
            }
        }
        match w.default {
            Dest::A => {
                accepted.push(p);
            }
            Dest::R => {}
            Dest::Forward(ref next) => {
                queue.push((p, next.clone()));
            }
        }
    }
    let mut ret = 0;
    for a in accepted {
        let mut combs = 1;
        for s in a.scores {
            combs *= s.max - s.min + 1;
        }
        ret += combs;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 19114);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 167409079868000);
    }
}
