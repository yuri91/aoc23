use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;
use pom::parser::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn digit<'a>() -> Parser<'a, u8, u32> {
    one_of(b"0123456789").map(|d| (d - b'0') as u32)
}
fn row<'a>() -> Parser<'a, u8, Vec<u32>> {
    digit().repeat(1..) - (sym(b'\n').discard() | end())
}
fn map<'a>() -> Parser<'a, u8, Vec<Vec<u32>>> {
    row().repeat(1..) - end()
}

#[aoc_generator(day17)]
fn input_gen(input: &[u8]) -> Result<Array2<u32>> {
    let data = map().parse(input)?;
    let shape = (data.len(), data[0].len());
    let flat = data.into_iter().flatten().collect();
    Ok(Array2::from_shape_vec(shape, flat)?)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}
fn neighbours(
    map: &Array2<u32>,
    (y, x): (usize, usize),
    dir: Dir,
    dir_count: usize,
    min_straight: usize,
    max_straight: usize,
) -> Vec<((usize, usize), Dir)> {
    let [h, w] = map.shape() else { unreachable!() };
    let straight_allowed = dir_count < max_straight;
    let turn_allowed = dir_count >= min_straight || dir_count == 0;
    let mut ret = Vec::new();
    if x > 0
        && (straight_allowed || dir != Dir::Left)
        && (turn_allowed || dir == Dir::Left)
        && dir != Dir::Right
    {
        ret.push(((y, x - 1), Dir::Left));
    }
    if x < w - 1
        && (straight_allowed || dir != Dir::Right)
        && (turn_allowed || dir == Dir::Right)
        && dir != Dir::Left
    {
        ret.push(((y, x + 1), Dir::Right));
    }
    if y > 0
        && (straight_allowed || dir != Dir::Up)
        && (turn_allowed || dir == Dir::Up)
        && dir != Dir::Down
    {
        ret.push(((y - 1, x), Dir::Up));
    }
    if y < h - 1
        && (straight_allowed || dir != Dir::Down)
        && (turn_allowed || dir == Dir::Down)
        && dir != Dir::Up
    {
        ret.push(((y + 1, x), Dir::Down));
    }
    ret
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct DistanceState {
    dist: u32,
    dir: Dir,
    dir_count: u32,
}

fn dijkstra_constraints(
    map: &Array2<u32>,
    from: (usize, usize),
    to: (usize, usize),
    min_straight: usize,
    max_straight: usize,
) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, from, Dir::Right, 0, vec![])));
    let mut visited = HashSet::new();
    while let Some(Reverse((p_d, p, p_dir, p_dir_count, preds))) = queue.pop() {
        if p == to && p_dir_count < min_straight {
            continue;
        }
        if visited.contains(&(p, p_dir, p_dir_count)) {
            continue;
        }
        if p == to {
            let mut pmap = map.clone();
            for pred in preds {
                pmap[pred] = 0;
            }
            pmap[to] = 0;
            return p_d;
        }
        visited.insert((p, p_dir, p_dir_count));
        for (n, n_dir) in neighbours(map, p, p_dir, p_dir_count, min_straight, max_straight) {
            let n_d = map[n] + p_d;
            let n_dir_count = if n_dir == p_dir { p_dir_count + 1 } else { 1 };
            let mut n_preds = preds.clone();
            n_preds.push(p);
            queue.push(Reverse((n_d, n, n_dir, n_dir_count, n_preds)));
        }
    }
    u32::MAX
}

#[aoc(day17, part1)]
fn part1(input: &Array2<u32>) -> u32 {
    let [h, w] = input.shape() else {
        unreachable!()
    };
    dijkstra_constraints(input, (0, 0), (h - 1, w - 1), 0, 3)
}

#[aoc(day17, part2)]
fn part2(input: &Array2<u32>) -> u32 {
    let [h, w] = input.shape() else {
        unreachable!()
    };
    dijkstra_constraints(input, (0, 0), (h - 1, w - 1), 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    const EXAMPLE2: &[u8] = br#"111111111111
999999999991
999999999991
999999999991
999999999991"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 102);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 94);
    }
    #[test]
    fn part2_example2() {
        assert_eq!(part2(&input_gen(EXAMPLE2).unwrap()), 71);
    }
}
