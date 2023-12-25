use std::collections::HashSet;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use parser::parse_file;
use petgraph::stable_graph::NodeIndex;

use crate::graph::{are_interconnected, is_connected_to_all};

mod graph;
mod parser;

pub fn part1(input: &'static str) -> Result<u64> {
    let (_, graph) = parse_file(input).expect("parse ok");
    let number_of_nodes = graph.node_count();

    // dbg!(&graph);
    // print_graph(&graph);
    // output_dot_file(&graph, "graph.dot");

    // get a random node and find a subgraph
    // where all members are interconnected by at least 3 edges.

    // expect we need 4-6 nodes to form such a seed subgraph
    let mut subgroup = 'seeding: {
        loop {
            let seed = rand::random::<usize>() % number_of_nodes;
            let initial = graph.node_indices().skip(seed).next().expect("node index");
            // dbg!(&initial);

            let mut subgroup: HashSet<NodeIndex> = HashSet::new();
            subgroup.insert(initial);
            for nbr in graph.neighbors_undirected(initial) {
                subgroup.insert(nbr);
            }
            for _ in 0..2 {
                for &node in &subgroup.iter().cloned().collect_vec() {
                    for nbr in graph.neighbors_undirected(node) {
                        subgroup.insert(nbr);
                    }
                }
            }

            break 'seeding subgroup;

            let mut candidates = HashSet::new();
            for nbr in graph.neighbors_undirected(initial) {
                candidates.insert(nbr);
            }
            // dbg!(&candidates);

            for c in candidates.iter().cloned().collect_vec() {
                for nbr in graph.neighbors_undirected(c) {
                    if nbr != initial {
                        candidates.insert(nbr);
                    }
                }
            }

            // for c in candidates.iter().cloned().collect_vec() {
            //     for nbr in graph.neighbors_undirected(c) {
            //         if nbr != initial {
            //             candidates.insert(nbr);
            //         }
            //     }
            // }

            assert!(candidates.len() > 2);
            for mut combo in candidates.iter().combinations(4) {
                // dbg!(&combo);

                combo.push(&initial);
                // dbg!(&combo);

                if are_interconnected(&graph, &combo) {
                    // dbg!(&combo);
                    subgroup.extend(combo.iter().map(|r| *r));
                    // subgroup.insert(initial);
                    break;
                }
            }

            if !subgroup.is_empty() {
                break 'seeding subgroup;
            }

            for mut combo in candidates.iter().combinations(8) {
                combo.push(&initial);
                if are_interconnected(&graph, &combo) {
                    subgroup.extend(combo.iter().map(|r| *r));

                    break;
                }
            }

            if !subgroup.is_empty() {
                break 'seeding subgroup;
            }

            for mut combo in candidates.iter().combinations(9) {
                combo.push(&initial);
                if are_interconnected(&graph, &combo) {
                    subgroup.extend(combo.iter().map(|r| *r));

                    break;
                }
            }

            if !subgroup.is_empty() {
                break 'seeding subgroup;
            }
        }
    };

    dbg!(subgroup.len());

    // repeatedly eval subgraph edges, connected node is merge into the subgraph
    // if it is connected to 3 or more of subgraph members
    // end the process when subgraph has only three outgoing connections left
    let mut connections = HashSet::new(); // outgoing edges from subgraph
    let mut contending = HashSet::new(); // immediate neighbours of subgraph

    for node in &subgroup {
        for nbr in graph.neighbors_undirected(*node) {
            // add outgoing edges
            let edge = graph.find_edge(*node, nbr).expect("edge");
            connections.insert(edge);

            // add contending node
            if !subgroup.contains(&nbr) {
                contending.insert(nbr);
            }
        }
    }
    for node in &subgroup {
        for nbr in graph.neighbors_undirected(*node) {
            if subgroup.contains(&nbr) {
                let edge = graph.find_edge(*node, nbr).expect("edge");
                connections.remove(&edge);
            }
        }
    }

    // dbg!(&contending);
    // dbg!(&connections);

    while connections.len() != 3 {
        // enumerate contending and add valid members to a list
        let mut valid_nodes = HashSet::new();
        for node in &contending {
            // if connected to at least 3 members of the subgroup
            // add to valid list
            // let mut count = 0;
            // for nbr in graph.neighbors(*node) {
            //     if subgroup.contains(&nbr) {
            //         count += 1;
            //     }
            // }
            // if count >= 3 {
            //     valid_nodes.insert(*node);
            //     valid_nodes.insert(*node);
            // }
            // for nbr in graph.neighbors_undirected(*node) {
            //     println!("{node:?} is connected to {nbr:?}");
            // }
            if is_connected_to_all(&graph, *node, subgroup.iter()) {
                // dbg!("!!");
                // 2 members only
                valid_nodes.insert(*node);
            }
        }
        // break;
        // expand contending search scope.
        if valid_nodes.is_empty() {
            for node in contending.iter().cloned().collect_vec() {
                for nbr in graph.neighbors_undirected(node) {
                    if !subgroup.contains(&nbr) {
                        contending.insert(nbr);
                    }
                }
            }
        } else {
            // Process valid nodes
            // 1) remove valid node from contending
            // 2a) update connections: add new node edges, cleanup intra subgraph edge in 3.
            // 2b) Add valid nodes to subgraph
            // 2c) Add non-subgraph nbrs of valid nodes to contending
            // 3) remove subgraph-node edges
            for node in &valid_nodes {
                contending.remove(node);
            }
            for node in &valid_nodes {
                for nbr in graph.neighbors_undirected(*node) {
                    // add outgoing edges
                    let edge = graph.find_edge(*node, nbr).expect("edge");
                    connections.insert(edge);

                    // add contending node
                    if !subgroup.contains(&nbr) && !valid_nodes.contains(&nbr) {
                        contending.insert(nbr);
                    }
                }
                subgroup.insert(*node);
            }
            for node in &valid_nodes {
                for nbr in graph.neighbors_undirected(*node) {
                    if subgroup.contains(&nbr) {
                        let edge = graph.find_edge(*node, nbr).expect("edge");
                        connections.remove(&edge);
                    }
                }
            }
            valid_nodes.drain();
        }

        // dbg!(&contending.len());
        // dbg!(&subgroup);

        // break;
    }

    // dbg!(&connections);
    // for con in &connections {
    //     let edge = graph.edge_endpoints(*con);
    //     dbg!(edge);
    // }
    // dbg!(&subgroup);

    let subgraph_size = subgroup.len();
    let another_size = number_of_nodes - subgraph_size;
    Ok((subgraph_size * another_size) as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(54)]
    fn part_1_using_test_input_1(#[case] expected: u64) {
        let result = part1(include_str!("../test-1.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(0)]
    fn part_2_using_test_input_1(#[case] expected: u64) {
        let result = part2(include_str!("../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
