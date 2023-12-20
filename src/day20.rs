use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{graph::NodeIndex, Direction, Graph};
use pom::parser::*;
use std::collections::HashMap;
use std::collections::VecDeque;

fn ident<'a>() -> Parser<'a, u8, String> {
    one_of(b"abcdefghijklmnopqrstuvwxyz")
        .repeat(1..)
        .collect()
        .convert(std::str::from_utf8)
        .map(|s| s.to_owned())
}

fn module<'a>() -> Parser<'a, u8, Module> {
    ((sym(b'%').map(|_| ModuleType::FlipFlop) | sym(b'&').map(|_| ModuleType::Conjunction))
        .opt()
        .map(|t| t.unwrap_or(ModuleType::None))
        + ident()
        - seq(b" -> ")
        + list(ident(), seq(b", "))
        - (sym(b'\n').discard() | end()))
    .map(|((ty, name), succs)| Module { ty, name, succs })
}

fn conf<'a>() -> Parser<'a, u8, Vec<Module>> {
    module().repeat(1..) - end()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Module {
    name: String,
    ty: ModuleType,
    succs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ModuleState {
    name: String,
    ty: ModuleType,
    state: Vec<bool>,
}
impl ModuleState {
    fn new(name: String, ty: ModuleType) -> ModuleState {
        ModuleState {
            name,
            ty,
            state: vec![],
        }
    }
    fn recv_pulse(&mut self, from: usize, v: bool) -> Option<bool> {
        match self.ty {
            ModuleType::FlipFlop => {
                if v {
                    return None;
                }
                self.state[0] = !self.state[0];
                Some(self.state[0])
            }
            ModuleType::Conjunction => {
                self.state[from] = v;
                Some(!self.state.iter().all(|&i| i))
            }
            ModuleType::None => Some(v),
        }
    }
}

#[aoc_generator(day20)]
fn input_gen(input: &[u8]) -> Result<Vec<Module>> {
    Ok(conf().parse(input)?)
}

fn make_graph(input: &[Module]) -> (Graph<ModuleState, ()>, HashMap<String, NodeIndex>) {
    let mut indices = HashMap::new();
    let mut g: Graph<ModuleState, ()> = Graph::new();
    for m in input {
        indices.insert(
            m.name.clone(),
            g.add_node(ModuleState::new(m.name.clone(), m.ty)),
        );
    }
    for m in input {
        let m_idx = indices[&m.name];
        for s in &m.succs {
            let s_idx = *indices
                .entry(s.clone())
                .or_insert_with(|| g.add_node(ModuleState::new(s.clone(), ModuleType::None)));
            g.add_edge(m_idx, s_idx, ());
            g[s_idx].state.push(false);
        }
    }
    (g, indices)
}
fn push_button(
    g: &mut Graph<ModuleState, ()>,
    broadcaster: NodeIndex,
    end: Option<NodeIndex>,
) -> (u64, u64, Option<bool>) {
    let mut queue = VecDeque::new();
    queue.push_back((broadcaster, false, 0));
    let mut high = 0;
    let mut low = 0;
    let mut end_pulse = None;
    while let Some((cur, in_pulse, from_idx)) = queue.pop_front() {
        if in_pulse {
            high += 1;
        } else {
            low += 1;
        }
        if let Some(out_pulse) = g[cur].recv_pulse(from_idx, in_pulse) {
            if Some(cur) == end && out_pulse {
                end_pulse = Some(out_pulse);
                break;
            }
            let mut succs = g.neighbors_directed(cur, Direction::Outgoing).detach();
            while let Some((_, s)) = succs.next(g) {
                let (s_from_idx, _) = g
                    .neighbors_directed(s, Direction::Incoming)
                    .enumerate()
                    .find(|&(_, p)| p == cur)
                    .unwrap();
                queue.push_back((s, out_pulse, s_from_idx));
            }
        }
    }
    (low, high, end_pulse)
}

fn find_period(g: &mut Graph<ModuleState, ()>, start: NodeIndex, end: NodeIndex) -> u64 {
    let mut i = 1;
    while push_button(g, start, Some(end)).2 != Some(true) {
        i += 1;
    }
    i
}

#[aoc(day20, part1)]
fn part1(input: &[Module]) -> u64 {
    let (mut g, indices) = make_graph(input);
    let broadcaster = indices["broadcaster"];
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h, _) = push_button(&mut g, broadcaster, None);
        low += l;
        high += h;
    }
    low * high
}

#[aoc(day20, part2)]
fn part2(input: &[Module]) -> u64 {
    let (g, indices) = make_graph(input);
    let broadcaster = indices["broadcaster"];
    let rx = indices["rx"];
    let pred = g
        .neighbors_directed(rx, Direction::Incoming)
        .next()
        .unwrap();
    assert!(
        g[pred].ty == ModuleType::Conjunction,
        "unexpected graph shape"
    );
    let incoming: Vec<_> = g.neighbors_directed(pred, Direction::Incoming).collect();
    let mut ret = 1;
    for i in incoming {
        let mut subg = g.clone();
        let p = find_period(&mut subg, broadcaster, i);
        ret = num::integer::lcm(ret, p);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = br#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
    const EXAMPLE2: &[u8] = br#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_gen(EXAMPLE).unwrap()), 32000000);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(part1(&input_gen(EXAMPLE2).unwrap()), 11687500);
    }
    #[test]
    fn part2_example() {}
}
