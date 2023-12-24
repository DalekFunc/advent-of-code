use std::{
    collections::{HashSet, VecDeque},
    str::pattern::Pattern,
};

use itertools::Itertools;
use petgraph::{
    graph::UnGraph, stable_graph::NodeIndex, visit::IntoNodeReferences, Graph, Undirected,
};

use crate::{coordinates::Coord, grid::Grid};

pub type Trails = Graph<Coord, u32>;
pub type Trails2 = Graph<Coord, u32, Undirected>;

#[derive(Debug)]
struct Record {
    cur: Coord,
    prev: Coord,
    begin: Coord,
    steps: u32,
}

// region: Helpers
fn get_node_index(graph: &mut Trails, pos: Coord) -> NodeIndex {
    match graph.node_references().find(|(_idx, coord)| **coord == pos) {
        Some((idx, _)) => idx,
        None => graph.add_node(pos),
    }
}

fn add_directed_edge(graph: &mut Trails, from: Coord, to: Coord, weight: u32) {
    // get node idx for from and to
    // create if not exists
    let from = get_node_index(graph, from);
    let to = get_node_index(graph, to);

    graph.update_edge(from, to, weight);
}

fn get_node_index2(graph: &mut Trails2, pos: Coord) -> NodeIndex {
    match graph.node_references().find(|(_idx, coord)| **coord == pos) {
        Some((idx, _)) => idx,
        None => graph.add_node(pos),
    }
}

fn add_directed_edge2(graph: &mut Trails2, from: Coord, to: Coord, weight: u32) {
    // get node idx for from and to
    // create if not exists
    let from = get_node_index2(graph, from);
    let to = get_node_index2(graph, to);

    graph.add_edge(from, to, weight);
}

// endregion: Helpers

// dfs directed graph generation
pub fn graph_generation<const N: usize>(map: &Grid<char, N>, start: Coord, end: Coord) -> Trails {
    let mut graph = Graph::new();
    graph.add_node(start);

    let record = Record {
        cur: start,
        prev: start,
        begin: start,
        steps: 0,
    };
    let mut queue = VecDeque::from([record]);

    while let Some(mut record) = queue.pop_front() {

        loop {
            let nbrs = record
                .cur
                .neighbours(map.row_bound(), map.col_bound())
                .into_iter()
                .filter(|&coord| coord != record.prev)
                .filter(|&nbr| map[nbr] != '#')
                .collect_vec();

            if nbrs.len() == 1 {
                // dot to arrow
                if map[nbrs[0]].is_contained_in("<v>") {
                    match map[nbrs[0]] {
                        '<' if nbrs[0].col > record.cur.col => break,
                        '>' if nbrs[0].col < record.cur.col => break,
                        'v' if nbrs[0].row < record.cur.row => break,
                        '<' | '>' | 'v' => {}
                        _ => unreachable!("invalid character"),
                    }
                    add_directed_edge(&mut graph, record.begin, nbrs[0], record.steps + 1);
                    queue.push_back(Record {
                        cur: nbrs[0],
                        prev: record.cur,
                        begin: nbrs[0],
                        steps: 0,
                    });
                    break;
                } else {
                    if nbrs[0] == end {
                        add_directed_edge(&mut graph, record.begin, nbrs[0], record.steps + 1);
                        break;
                    }
                    // dot to dot
                    record.prev = record.cur;
                    record.cur = nbrs[0];
                    record.steps += 1;
                }
            } else {
                // arrow to conjunction
                assert!(map[record.cur] == '.');
                for nbr in nbrs {
                    match map[nbr] {
                        '<' if nbr.col > record.cur.col => continue,
                        '>' if nbr.col < record.cur.col => continue,
                        'v' if nbr.row < record.cur.row => continue,
                        '<' | '>' | 'v' => {}
                        c => unreachable!("invalid character {}", c),
                    }

                    add_directed_edge(&mut graph, record.begin, nbr, record.steps + 1);
                    queue.push_back(Record {
                        cur: nbr,
                        prev: record.cur,
                        begin: nbr,
                        steps: 0,
                    });
                }
                break;
            }
        }
    }
    graph
}

// dfs directed graph generation
pub fn graph_generation2<const N: usize>(map: &Grid<char, N>, start: Coord, end: Coord) -> Trails2 {
    let mut graph = Graph::new_undirected();
    graph.add_node(start);

    let mut queue = VecDeque::from([(start, start, 0)]);
    let mut visited = HashSet::new();
    while let Some((mut current, mut from, mut steps)) = queue.pop_front() {
        loop {
            // dbg!(current);
            if visited.contains(&current) {
                add_directed_edge2(&mut graph, from, current, steps);
                break;
            }
            visited.insert(current);

            let nbrs = current
                .neighbours(map.row_bound(), map.col_bound())
                .into_iter()
                .filter(|coord| !visited.contains(coord))
                .filter(|&coord| map[coord] != '#')
                .collect_vec();
            // dbg!(&nbrs);

            if nbrs.len() == 0 {
                if current == end {
                    add_directed_edge2(&mut graph, from, current, steps);
                    steps = 1;
                    break;
                }
            } else if nbrs.len() == 1 {
                current = nbrs[0];
                steps += 1;
            } else {
                add_directed_edge2(&mut graph, from, current, steps);
                for nbr in &nbrs[1..] {
                    queue.push_back((*nbr, current, 1));
                }

                from = current;
                current = nbrs[0];
                steps = 1;
            }
        }
    }
    graph
}
