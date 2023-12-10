use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use pom::parser::*;
use std::collections::HashMap;

fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'|').map(|_| Tile::V)
        | sym(b'-').map(|_| Tile::H)
        | sym(b'L').map(|_| Tile::L)
        | sym(b'J').map(|_| Tile::J)
        | sym(b'7').map(|_| Tile::N7)
        | sym(b'F').map(|_| Tile::F)
        | sym(b'.').map(|_| Tile::E)
        | sym(b'S').map(|_| Tile::S)
}
fn row<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}
fn map<'a>() -> Parser<'a, u8, Vec<Vec<Tile>>> {
    row().repeat(1..)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    V,
    H,
    L,
    J,
    N7,
    F,
    E,
    S,
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<Tile>>,
}

#[aoc_generator(day10)]
fn input_gen(input: &[u8]) -> Result<Map> {
    Ok(Map {
        data: map().parse(input)?,
    })
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Node {
    x: usize,
    y: usize,
    tile: Tile,
}

fn build_graph(map: &Map) -> (UnGraph<Node, ()>, NodeIndex) {
    use Tile::*;
    let mut s_id = None;
    let mut g = UnGraph::<Node, ()>::new_undirected();
    let mut ids = HashMap::new();
    for (y, row) in map.data.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            let n = Node { x, y, tile: *t };
            let id = g.add_node(n);
            if *t == S {
                s_id = Some(id);
            }
            ids.insert((x, y), id);
            if y > 0 {
                let neigh_id = ids.get(&(n.x, n.y - 1)).unwrap();
                let nt = map.data[y - 1][x];
                if let (V | L | J | S, V | N7 | F | S) = (t, nt) {
                    g.add_edge(id, *neigh_id, ());
                }
            }
            if x > 0 {
                let neigh_id = ids.get(&(n.x - 1, n.y)).unwrap();
                let nt = map.data[y][x - 1];
                if let (H | J | N7 | S, H | L | F | S) = (t, nt) {
                    g.add_edge(id, *neigh_id, ());
                }
            }
        }
    }
    (g, s_id.unwrap())
}
fn get_loop(g: &UnGraph<Node, ()>, s_id: NodeIndex) -> Vec<NodeIndex> {
    let sccs = petgraph::algo::kosaraju_scc(&g);
    for scc in sccs {
        if scc.len() > 1 && scc.contains(&s_id) {
            return scc;
        }
    }
    Vec::new()
}

#[aoc(day10, part1)]
fn part1(input: &Map) -> i64 {
    let (g, s_id) = build_graph(input);
    let scc = get_loop(&g, s_id);
    let dists = petgraph::algo::dijkstra(&g, s_id, None, |_| 1);
    *scc.iter()
        .map(|n| dists.get(n).unwrap())
        .sorted()
        .next_back()
        .unwrap()
}

fn get_s_tile(g: &UnGraph<Node, ()>, s: NodeIndex) -> Tile {
    let (i1, i2) = g.neighbors(s).collect_tuple().unwrap();
    let n1 = g.node_weight(i1).unwrap();
    let n2 = g.node_weight(i2).unwrap();
    let ns = g.node_weight(s).unwrap();
    let (n1, n2) = if (n1.y, n1.x) < (n2.y, n2.x) {
        (n1, n2)
    } else {
        (n2, n1)
    };
    enum Side {
        Top,
        Bottom,
        Left,
        Right,
    }
    use Side::*;
    use Tile::*;
    let s1 = if n1.y < ns.y {
        Top
    } else if n1.x < ns.x {
        Left
    } else {
        Right
    };
    let s2 = if n2.y > ns.y {
        Bottom
    } else if n2.x > ns.x {
        Right
    } else {
        Left
    };
    match (s1, s2) {
        (Top, Right) => L,
        (Top, Bottom) => V,
        (Left, Right) => H,
        (Left, Bottom) => N7,
        (Right, Bottom) => F,
        (Top, Left) => J,
        _ => unreachable!(),
    }
}

#[aoc(day10, part2)]
fn part2(input: &Map) -> i64 {
    let (g, s_id) = build_graph(input);
    let scc = get_loop(&g, s_id);
    let nodes = scc
        .iter()
        .map(|id| *g.node_weight(*id).unwrap())
        .collect_vec();
    let mut new_map = vec![vec![Tile::E; input.data[0].len()]; input.data.len()];
    for n in nodes {
        new_map[n.y][n.x] = if n.tile == Tile::S {
            get_s_tile(&g, s_id)
        } else {
            n.tile
        };
    }
    let mut ret = 0;
    for y in 0..new_map.len() {
        let mut n = 0;
        for x in 0..new_map[0].len() {
            match new_map[y][x] {
                Tile::E => {
                    if n % 2 == 1 {
                        ret += 1;
                    }
                }
                Tile::V | Tile::J | Tile::L => {
                    n += 1;
                }
                _ => {}
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &[u8] = br#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    const EXAMPLE2: &[u8] = br#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE1).unwrap()), 8);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE2).unwrap()), 10);
    }
}
