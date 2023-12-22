use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{s, Array3};
use pom::parser::*;

fn unsigned<'a>() -> Parser<'a, u8, usize> {
    let n = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    n.collect()
        .convert(std::str::from_utf8)
        .convert(|s| s.parse())
}

fn pos<'a>() -> Parser<'a, u8, Pos> {
    (unsigned() - sym(b',') + unsigned() - sym(b',') + unsigned()).map(|((x, y), z)| Pos {
        x,
        y,
        z,
    })
}

fn brick<'a>() -> Parser<'a, u8, Brick> {
    (pos() - sym(b'~') + pos() - (sym(b'\n').discard() | end()))
        .map(|(from, to)| Brick { from, to })
}

fn bricks<'a>() -> Parser<'a, u8, Vec<Brick>> {
    brick().repeat(1..) - end()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Brick {
    from: Pos,
    to: Pos,
}
impl Brick {
    fn falling(&self, map: &Array3<bool>) -> bool {
        let base = map.slice(s![
            self.from.x..=self.to.x,
            self.from.y..=self.to.y,
            self.from.z - 1
        ]);
        !base.iter().any(|b| *b)
    }
    fn blit(&self, map: &mut Array3<bool>, value: bool) {
        let tiles = map.slice_mut(s![
            self.from.x..=self.to.x,
            self.from.y..=self.to.y,
            self.from.z..=self.to.z
        ]);
        for t in tiles {
            *t = value;
        }
    }
    fn fall(&mut self, map: &mut Array3<bool>) {
        assert!(self.from.z > 0);
        self.blit(map, false);
        self.from.z -= 1;
        self.to.z -= 1;
        self.blit(map, true);
    }
}

fn fall(bricks: &mut [Brick], map: &mut Array3<bool>) -> usize {
    let mut ret = 0;
    for br in bricks {
        let mut fell = false;
        while br.falling(map) {
            br.fall(map);
            fell = true;
        }
        if fell {
            ret += 1;
        }
    }
    ret
}

fn init(input: &[Brick]) -> (Vec<Brick>, Array3<bool>) {
    let mut bricks: Vec<_> = input.to_vec();
    bricks.sort_by_key(|b| b.from.z);
    let max_x = input.iter().map(|b| b.to.x.max(b.from.x)).max().unwrap();
    let max_y = input.iter().map(|b| b.to.y.max(b.from.y)).max().unwrap();
    let max_z = input.iter().map(|b| b.to.z.max(b.from.z)).max().unwrap();

    let mut map = Array3::from_shape_fn((max_x + 1, max_y + 1, max_z + 1), |(_, _, z)| z == 0);

    for br in &mut bricks {
        br.blit(&mut map, true);
    }

    fall(&mut bricks, &mut map);

    (bricks, map)
}

#[aoc_generator(day22)]
fn input_gen(input: &[u8]) -> Result<Vec<Brick>> {
    Ok(bricks().parse(input)?)
}

#[aoc(day22, part1)]
fn part1(input: &[Brick]) -> u32 {
    let (bricks, map) = init(input);
    let mut ret = 0;
    for i in 0..bricks.len() {
        let mut sym_bricks = bricks.clone();
        let mut sym_map = map.clone();
        sym_bricks[i].blit(&mut sym_map, false);
        sym_bricks.remove(i);
        if fall(&mut sym_bricks, &mut sym_map) == 0 {
            ret += 1;
        }
    }
    ret
}

#[aoc(day22, part2)]
fn part2(input: &[Brick]) -> usize {
    let (bricks, map) = init(input);
    let mut ret = 0;
    for i in 0..bricks.len() {
        let mut sym_bricks = bricks.clone();
        let mut sym_map = map.clone();
        sym_bricks[i].blit(&mut sym_map, false);
        sym_bricks.remove(i);
        ret += fall(&mut sym_bricks, &mut sym_map);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 5);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 7);
    }
}
