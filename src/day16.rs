use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use enumflags2::{bitflags, BitFlags};
use pom::parser::*;

fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'.').map(|_| Tile::Empty)
        | sym(b'|').map(|_| Tile::VSplit)
        | sym(b'-').map(|_| Tile::HSplit)
        | sym(b'\\').map(|_| Tile::LMirror)
        | sym(b'/').map(|_| Tile::RMirror)
}
fn row<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}
fn map<'a>() -> Parser<'a, u8, Vec<Vec<Tile>>> {
    row().repeat(1..) - end()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    VSplit,
    HSplit,
    LMirror,
    RMirror,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BeamDir {
    Up = 0b0001,
    Right = 0b0010,
    Down = 0b0100,
    Left = 0b1000,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    ty: Tile,
    beams: BitFlags<BeamDir>,
}

#[aoc_generator(day16)]
fn input_gen(input: &[u8]) -> Result<Vec<Vec<Tile>>> {
    let seq = map().parse(input)?;
    Ok(seq)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Beam {
    x: usize,
    y: usize,
    dir: BeamDir,
}
impl Beam {
    fn advance(&mut self, w: usize, h: usize) -> bool {
        match self.dir {
            BeamDir::Right => {
                if self.x >= w - 1 {
                    return false;
                }
                self.x += 1;
            }
            BeamDir::Left => {
                if self.x == 0 {
                    return false;
                }
                self.x -= 1;
            }
            BeamDir::Down => {
                if self.y >= h - 1 {
                    return false;
                }
                self.y += 1;
            }
            BeamDir::Up => {
                if self.y == 0 {
                    return false;
                }
                self.y -= 1;
            }
        }
        true
    }
}

fn energize(mut map: Vec<Vec<State>>, b: Beam) -> i64 {
    let w = map[0].len();
    let h = map.len();
    let mut stack = vec![b];
    let mut n_energized = 0;
    while let Some(mut b) = stack.pop() {
        let s = &mut map[b.y][b.x];
        if s.beams.contains(b.dir) {
            continue;
        }
        if s.beams.is_empty() {
            n_energized += 1;
        }
        s.beams |= b.dir;
        match s.ty {
            Tile::VSplit if (BeamDir::Right | BeamDir::Left).contains(b.dir) => {
                let mut b1 = b;
                b1.dir = BeamDir::Up;
                if b1.advance(w, h) {
                    stack.push(b1);
                }
                let mut b2 = b;
                b2.dir = BeamDir::Down;
                if b2.advance(w, h) {
                    stack.push(b2);
                }
            }
            Tile::HSplit if (BeamDir::Up | BeamDir::Down).contains(b.dir) => {
                let mut b1 = b;
                b1.dir = BeamDir::Right;
                if b1.advance(w, h) {
                    stack.push(b1);
                }
                let mut b2 = b;
                b2.dir = BeamDir::Left;
                if b2.advance(w, h) {
                    stack.push(b2);
                }
            }
            Tile::LMirror => {
                b.dir = match b.dir {
                    BeamDir::Left => BeamDir::Up,
                    BeamDir::Up => BeamDir::Left,
                    BeamDir::Right => BeamDir::Down,
                    BeamDir::Down => BeamDir::Right,
                };
                if b.advance(w, h) {
                    stack.push(b);
                }
            }
            Tile::RMirror => {
                b.dir = match b.dir {
                    BeamDir::Left => BeamDir::Down,
                    BeamDir::Up => BeamDir::Right,
                    BeamDir::Right => BeamDir::Up,
                    BeamDir::Down => BeamDir::Left,
                };
                if b.advance(w, h) {
                    stack.push(b);
                }
            }
            _ => {
                if b.advance(w, h) {
                    stack.push(b);
                }
            }
        }
    }
    n_energized
}
#[aoc(day16, part1)]
fn part1(input: &[Vec<Tile>]) -> i64 {
    let map: Vec<Vec<State>> = input
        .iter()
        .map(|row| {
            row.iter()
                .map(|&ty| State {
                    ty,
                    beams: BitFlags::default(),
                })
                .collect()
        })
        .collect();
    energize(
        map,
        Beam {
            x: 0,
            y: 0,
            dir: BeamDir::Right,
        },
    )
}

#[aoc(day16, part2)]
fn part2(input: &[Vec<Tile>]) -> i64 {
    let map: Vec<Vec<State>> = input
        .iter()
        .map(|row| {
            row.iter()
                .map(|&ty| State {
                    ty,
                    beams: BitFlags::default(),
                })
                .collect()
        })
        .collect();
    let w = map[0].len();
    let h = map.len();
    let mut max = 0;
    for y in 0..h {
        max = max.max(energize(
            map.clone(),
            Beam {
                x: 0,
                y,
                dir: BeamDir::Right,
            },
        ));
        max = max.max(energize(
            map.clone(),
            Beam {
                x: w - 1,
                y,
                dir: BeamDir::Left,
            },
        ));
    }
    for x in 0..w {
        max = max.max(energize(
            map.clone(),
            Beam {
                x,
                y: 0,
                dir: BeamDir::Down,
            },
        ));
        max = max.max(energize(
            map.clone(),
            Beam {
                x,
                y: h - 1,
                dir: BeamDir::Up,
            },
        ));
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 46);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 51);
    }
}
