use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use pom::parser::*;
use std::collections::{HashMap, HashSet};

type Graph = UnGraph<usize, usize>;

fn ident<'a>() -> Parser<'a, u8, String> {
    one_of(b"abcdefghijklmnopqrstuvwxyz")
        .repeat(1..)
        .collect()
        .convert(std::str::from_utf8)
        .map(|s| s.to_owned())
}

fn item<'a>() -> Parser<'a, u8, (String, Vec<String>)> {
    ident() - seq(b": ") + list(ident(), sym(b' ')) - (sym(b'\n').discard() | end())
}

fn items<'a>() -> Parser<'a, u8, Vec<(String, Vec<String>)>> {
    item().repeat(1..) - end()
}

#[aoc_generator(day25)]
fn input_gen(input: &[u8]) -> Result<Graph> {
    let list = items().parse(input)?;
    let mut g = Graph::new_undirected();
    let mut indices = HashMap::new();
    let mut i = 0;
    for (from, tos) in list {
        let from_idx = *indices.entry(from).or_insert_with(|| {
            i += 1;
            g.add_node(1)
        });
        for to in tos {
            let to_idx = *indices.entry(to).or_insert_with(|| {
                i += 1;
                g.add_node(1)
            });
            g.add_edge(from_idx, to_idx, 1);
        }
    }
    Ok(g)
}

fn merge_nodes(g: &mut Graph, a: NodeIndex, b: NodeIndex) {
    let w1 = g.node_weight(a).unwrap();
    let w2 = g.node_weight(b).unwrap();
    let m = g.add_node(w1 + w2);
    for n in g.neighbors(a).collect::<Vec<_>>() {
        if n == b {
            continue;
        }
        let e = g.find_edge(a, n).unwrap();
        let w = *g.edge_weight(e).unwrap();
        g.add_edge(m, n, w);
    }
    for n in g.neighbors(b).collect::<Vec<_>>() {
        if n == a {
            continue;
        }
        let e = g.find_edge(b, n).unwrap();
        let mut w = *g.edge_weight(e).unwrap();
        if let Some(ae) = g.find_edge(a, n) {
            w += *g.edge_weight(ae).unwrap();
        }
        g.update_edge(m, n, w);
    }
    g.remove_node(a);
    g.remove_node(b);
}

fn most_tightly_connected(g: &Graph, set: &HashSet<NodeIndex>) -> NodeIndex {
    let mut ret = None;
    let mut w = 0;
    for n in g.node_indices() {
        let mut w_n = 0;
        if set.contains(&n) {
            continue;
        }
        for e in g.edges(n) {
            if !set.contains(&e.target()) {
                continue;
            }
            w_n += *e.weight();
        }
        if w < w_n {
            w = w_n;
            ret = Some(n);
        }
    }
    ret.unwrap()
}

fn minimum_cut_phase(g: &mut Graph, a: NodeIndex) -> (usize, usize, usize) {
    let mut set = HashSet::new();
    let mut order = vec![a];
    set.insert(a);
    while set.len() != g.node_count() {
        let m = most_tightly_connected(g, &set);
        set.insert(m);
        order.push(m);
    }
    let &[.., n1, n2] = order.as_slice() else {
        unreachable!()
    };
    let cut = g.edges(n2).map(|e| *e.weight()).sum();
    let c1 = *g.node_weight(n2).unwrap();
    let c2 = set
        .into_iter()
        .filter(|&n| n != n2)
        .map(|n| *g.node_weight(n).unwrap())
        .sum();
    merge_nodes(g, n1, n2);
    (cut, c1, c2)
}

fn minimum_cut(g: &mut Graph, stop_at: usize) -> (usize, usize, usize) {
    let mut min = usize::MAX;
    let mut s1 = 0;
    let mut s2 = 0;
    while g.node_count() > 1 {
        let a = g.node_indices().next().unwrap();
        let (cut, c1, c2) = minimum_cut_phase(g, a);
        if min > cut {
            min = cut;
            s1 = c1;
            s2 = c2;
        }
        if min <= stop_at {
            break;
        }
    }
    (min, s1, s2)
}

#[aoc(day25, part1)]
fn part1(input: &Graph) -> usize {
    let mut g = input.clone();
    let (_, c1, c2) = minimum_cut(&mut g, 3);
    c1 * c2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 54);
    }
}
