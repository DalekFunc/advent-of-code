#![feature(pattern)]

use std::{
    collections::{HashSet, VecDeque},
    ops::{Range, RangeInclusive},
    str::pattern::Pattern,
};

use anyhow::{anyhow, Result};
use coordinates::Coord;
use grid::Grid;
use itertools::Itertools;
use petgraph::{
    algo::all_simple_paths,
    dot::{Config, Dot},
    visit::IntoNodeReferences,
};
use trail::graph_generation;

use crate::trail::graph_generation2;

mod coordinates;
mod grid;
mod trail;

pub fn part1<const N: usize>(input: &str) -> Result<u64> {
    // FIXME: This possibly uses 2x to 3x more memory allocations...
    let mut map = Grid::<_, N>::new();
    input.split("\n").enumerate().for_each(|(row, line)| {
        let chars: Vec<_> = line.chars().collect();
        map[row].copy_from_slice(&chars);
    });

    let start = Coord {
        row: 0,
        col: map[0]
            .iter()
            .position(|&elem| elem == '.')
            .expect("starting"),
    };

    let end = Coord {
        row: N - 1,
        col: map[N - 1]
            .iter()
            .position(|&elem| elem == '.')
            .expect("ending"),
    };

    let graph = graph_generation(&map, start, end);
    let from = graph
        .node_references()
        .find(|(_, co)| **co == start)
        .expect("start")
        .0;
    let to = graph
        .node_references()
        .find(|(_, co)| **co == end)
        .expect("end")
        .0;

    let paths = all_simple_paths::<Vec<_>, _>(&graph, from, to, 0, None);

    println!("{:?}", Dot::with_config(&graph, &[]));

    let mut max_length = 0;
    for path in paths {
        let mut steps_total = 0;
        for (from, to) in path.iter().tuple_windows() {
            let edge = graph.find_edge(*from, *to).expect("edge");
            steps_total += graph.edge_weight(edge).expect("edge weight");
        }

        if steps_total > max_length {
            max_length = steps_total;
        }
    }

    Ok(max_length as u64)
}

pub fn part2<const N: usize>(input: &str) -> Result<u64> {
    let mut map = Grid::<_, N>::new();
    input.split("\n").enumerate().for_each(|(row, line)| {
        let chars: Vec<_> = line.chars().collect();
        map[row].copy_from_slice(&chars);
    });

    for row in map.row_bound() {
        for col in map.col_bound() {
            if map[Coord { row, col }].is_contained_in("<>v") {
                map[Coord { row, col }] = '.';
            }
        }
    }

    let start = Coord {
        row: 0,
        col: map[0]
            .iter()
            .position(|&elem| elem == '.')
            .expect("starting"),
    };

    let end = Coord {
        row: N - 1,
        col: map[N - 1]
            .iter()
            .position(|&elem| elem == '.')
            .expect("ending"),
    };

    let graph = graph_generation2(&map, start, end);
    let from = graph
        .node_references()
        .find(|(_, co)| **co == start)
        .expect("start")
        .0;
    let to = graph
        .node_references()
        .find(|(_, co)| **co == end)
        .expect("end")
        .0;

    let paths = all_simple_paths::<Vec<_>, _>(&graph, from, to, 0, None);

    println!("{:?}", Dot::with_config(&graph, &[]));

    let mut max_length = 0;

    for path in paths {
        let mut steps_total = 0;
        for (from, to) in path.iter().tuple_windows() {
            let edge = graph.find_edge(*from, *to).expect("edge");
            steps_total += graph.edge_weight(edge).expect("edge weight");
        }

        if steps_total > max_length {
            max_length = steps_total;
        }
    }

    Ok(max_length as u64)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(94)]
    fn part1_using_test_input_1(#[case] expected: u64) {
        let result = part1::<23>(include_str!("../test-1.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(14)]
    fn part1_using_test_input_2(#[case] expected: u64) {
        let result = part1::<10>(include_str!("../test-2.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(154)]
    fn part2_using_test_input_1(#[case] expected: u64) {
        let result = part2::<23>(include_str!("../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(14)]
    fn part2_using_test_input_2(#[case] expected: u64) {
        let result = part2::<10>(include_str!("../test-2.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(21)]
    fn part2_using_test_input_3(#[case] expected: u64) {
        let result = part2::<10>(include_str!("../test-3.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
