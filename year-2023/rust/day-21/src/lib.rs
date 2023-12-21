use std::{collections::HashSet, cmp::Reverse};

use anyhow::{anyhow, Result};
use coordinates::Coord;
use grid::Grid;
use parser::parse_map;
use priority_queue::PriorityQueue;

mod coordinates;
mod grid;
mod parser;

pub fn part1<const N: usize>(input: &str, step_limit: usize) -> Result<u64> {
    let grid = grid::<N>(input);
    // println!("{grid}");

    // find start
    let start = grid.find(&'S').expect("S");
    println!("{start:?}");

    // dijkstra
    // mark all pos reachable from steps % 2 == 0
    let reachables = dijkstra::<N>(&grid, start, step_limit);
    dbg!(&reachables);
    dbg!(&reachables.len());

    let mut marked = grid.clone();
    for pos in &reachables {
        marked[*pos] = 'O';
    }
    println!("{marked}");

    Ok(reachables.len() as u64)
}

pub fn part2(input: &str, step_limit: usize) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

fn grid<const N: usize>(input: &str) -> Grid<char, N> {
    let (_, map) = parse_map(input).expect("parse ok");

    let mut grid = Grid::<_, N>::new();
    for row in 0..N {
        for col in 0..N {
            grid[(row, col).into()] = map[row][col];
        }
    }

    grid
}

fn dijkstra<const N: usize>(grid: &Grid<char, N>, start: Coord, step_limit: usize) -> HashSet<Coord> {
    let mut frontier = PriorityQueue::new();
    frontier.push(start, Reverse(0));

    let mut reachable = HashSet::new();

    let mut cost_so_far = Grid::<Option<usize>, N>::new();
    cost_so_far[start] = Some(0);

    while !frontier.is_empty() {
        let (current, steps) = frontier.pop().expect("current");
        dbg!(current);
        dbg!(steps);

        if steps.0 % 2 == 0 {
            reachable.insert(current);
        }

        if steps.0 == step_limit + 1 {
            break;
        }

        for nbr in current.neighbours(N, N) {
            if grid[nbr] == '#' {
                continue;
            }

            let new_cost = cost_so_far[current].expect("current cost is not none") + 1;
            if cost_so_far[nbr].is_none()
                || new_cost < cost_so_far[nbr].expect("cost_so_far is not none")
            {
                cost_so_far[nbr] = Some(new_cost);

                frontier.push(nbr, Reverse(new_cost));
            }
        }
    }

    reachable
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(16)]
    fn part1_using_test_input_1(#[case] expected: u64) {
        let result = part1::<11>(include_str!("../test-1.txt"), 6).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(16, 6)]
    #[case(50, 10)]
    #[case(1594, 50)]
    #[case(6536, 100)]
    #[case(167004, 500)]
    #[case(668697, 1000)]
    #[case(16733044, 5000)]
    fn part2_using_test_input_2(#[case] expected: u64, #[case] step_limit: usize) {
        let result = part2(include_str!("../test-2.txt"), step_limit).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }

    #[test]
    fn quick_test() {}
}
