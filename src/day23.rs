use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use pom::parser::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

fn tile<'a>() -> Parser<'a, u8, Tile> {
    sym(b'.').map(|_| Tile::Path)
        | sym(b'#').map(|_| Tile::Forest)
        | sym(b'>').map(|_| Tile::SlopeR)
        | sym(b'<').map(|_| Tile::SlopeL)
        | sym(b'^').map(|_| Tile::SlopeU)
        | sym(b'v').map(|_| Tile::SlopeD)
}
fn row<'a>() -> Parser<'a, u8, Vec<Tile>> {
    tile().repeat(1..) - (sym(b'\n').discard() | end())
}
fn map<'a>() -> Parser<'a, u8, Vec<Vec<Tile>>> {
    row().repeat(1..)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Forest,
    Path,
    SlopeL,
    SlopeR,
    SlopeU,
    SlopeD,
}

#[aoc_generator(day23)]
fn input_gen(input: &[u8]) -> Result<Vec<Vec<Tile>>> {
    Ok(map().parse(input)?)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Node {
    x: usize,
    y: usize,
    tile: Tile,
}

fn neighbours<'a>(t: &'a Node, map: &'a [Vec<Tile>]) -> impl Iterator<Item = Node> + 'a {
    let h = map.len() as i64;
    let w = map[0].len() as i64;
    [
        (t.y as i64 - 1, t.x as i64),
        (t.y as i64 + 1, t.x as i64),
        (t.y as i64, t.x as i64 - 1),
        (t.y as i64, t.x as i64 + 1),
    ]
    .into_iter()
    .filter(move |&(y, x)| y > 0 && x > 0 && y < h && x < w)
    .map(move |(y, x)| {
        let x = x as usize;
        let y = y as usize;
        Node {
            x,
            y,
            tile: map[y][x],
        }
    })
    .filter(|n| n.tile != Tile::Forest)
    .filter(move |n| match t.tile {
        Tile::SlopeL => n.x < t.x,
        Tile::SlopeR => n.x > t.x,
        Tile::SlopeU => n.y < t.y,
        Tile::SlopeD => n.y > t.y,
        _ => true,
    })
}

fn build_graph(map: &[Vec<Tile>]) -> (Graph<Node, usize>, NodeIndex, NodeIndex) {
    let mut s_id = None;
    let mut d_id = None;
    let mut g = Graph::<Node, usize>::new();
    let mut ids = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if *t == Tile::Forest {
                continue;
            }
            let n = Node { x, y, tile: *t };
            let id = g.add_node(n);
            ids.insert((x, y), id);
            if y == 0 && *t == Tile::Path {
                s_id = Some(id);
            } else if y == map.len() - 1 && *t == Tile::Path {
                d_id = Some(id);
            }
        }
    }
    for (y, row) in map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if *t == Tile::Forest {
                continue;
            }
            let node = Node { x, y, tile: *t };
            let node_id = ids[&(x, y)];
            for neigh in neighbours(&node, map) {
                let neigh_id = ids[&(neigh.x, neigh.y)];
                g.add_edge(node_id, neigh_id, 1);
            }
        }
    }
    (g, s_id.unwrap(), d_id.unwrap())
}

fn build_graph2(map: &[Vec<Tile>]) -> (UnGraph<Node, usize>, NodeIndex, NodeIndex) {
    let mut g = UnGraph::<Node, usize>::new_undirected();
    let mut ids = HashMap::new();
    let mut s = None;
    let mut d = None;
    for (y, row) in map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if *t == Tile::Forest {
                continue;
            }
            let n = Node { x, y, tile: *t };
            if y == 0 && *t == Tile::Path {
                s = Some(n);
            } else if y == map.len() - 1 && *t == Tile::Path {
                d = Some(n);
            }
        }
    }
    let s = s.unwrap();
    let d = d.unwrap();
    let s_id = g.add_node(s);
    let d_id = g.add_node(d);
    ids.insert(s, s_id);
    ids.insert(d, d_id);
    let mut queue = VecDeque::new();
    for e in neighbours(&s, map) {
        queue.push_back((s, e));
    }
    let mut visited = HashSet::new();
    while let Some((from, to)) = queue.pop_front() {
        if visited.contains(&to) {
            continue;
        }
        visited.insert(to);
        let mut cur_from = from;
        let mut cur_to = to;
        let mut cur_w = 1;
        let hub = from;
        loop {
            let succs: Vec<_> = neighbours(&cur_to, map)
                .filter(|e| *e != cur_from)
                .map(|e| (cur_to, e))
                .collect();
            let count = succs.len();
            if count == 1 && succs[0].1 != d {
                let succ = succs[0];
                cur_from = succ.0;
                cur_to = succ.1;
                cur_w += 1;
                continue;
            } else {
                let hub_id = *ids.entry(hub).or_insert_with(|| g.add_node(hub));
                let cur_to_id = *ids.entry(cur_to).or_insert_with(|| g.add_node(cur_to));
                if g.find_edge(hub_id, cur_to_id).is_none() {
                    g.add_edge(hub_id, cur_to_id, cur_w);
                }
                for succ in succs {
                    queue.push_back(succ);
                }
                break;
            }
        }
    }
    (g, s_id, d_id)
}

#[aoc(day23, part1)]
fn part1(input: &[Vec<Tile>]) -> usize {
    let (g, s_id, d_id) = build_graph(input);

    let paths = petgraph::algo::all_simple_paths::<Vec<_>, _>(&g, s_id, d_id, 0, None);
    paths.map(|p| p.len()).max().unwrap() - 1
}

#[aoc(day23, part2)]
fn part2(input: &[Vec<Tile>]) -> usize {
    let no_slopes: Vec<Vec<Tile>> = input
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| match t {
                    Tile::Forest => Tile::Forest,
                    _ => Tile::Path,
                })
                .collect()
        })
        .collect();
    let (g, s_id, d_id) = build_graph2(&no_slopes);

    let paths = petgraph::algo::all_simple_paths::<Vec<_>, _>(&g, s_id, d_id, 0, None);
    paths
        .map(|p| {
            p.windows(2)
                .map(|w| g.edges_connecting(w[0], w[1]).next().unwrap().weight())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 94);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_gen(EXAMPLE).unwrap()), 154);
    }
}
