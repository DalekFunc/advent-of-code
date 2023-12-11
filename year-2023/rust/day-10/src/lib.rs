use std::collections::HashSet;

use anyhow::Result;
use coordinates::{neighbour_coord_in_direction, Coord};
use direction::Direction;
use itertools::Itertools;
use pipe::{C7, F, J, L, PIPE};

use crate::pipe::{connected, NEWLINE, S};

mod coordinates;
mod direction;
mod pipe;

pub fn part1(input: &[u8]) -> Result<u64> {
    let grid: Vec<&[u8]> = input.split(|b| *b == NEWLINE).collect();

    let start_coord = find_start(&grid);

    let (steps, _) = trace_loop(&grid, start_coord);

    Ok(steps / 2)
}

pub fn part2(input: &[u8]) -> Result<u64> {
    let grid: Vec<&[u8]> = input.split(|b| *b == NEWLINE).collect();

    let start_coord = find_start(&grid);

    let (_, loop_coords) = trace_loop(&grid, start_coord);

    // TODO: automatic modifiy starting point as a proper pipe;
    // let grid: Vec<&[u8]> = include_bytes!("../test-3-2.txt")
    // let grid: Vec<&[u8]> = include_bytes!("../test-4-2.txt")
    // let grid: Vec<&[u8]> = include_bytes!("../test-5-2.txt")
    let grid: Vec<&[u8]> = include_bytes!("../input-2.txt")
        .split(|b| *b == NEWLINE)
        .collect();

    // for each cell on each row, we look into the right direction
    // if the number of actual loop vertial crossing is odd, then it has to be inside.
    Ok((0..grid.len())
        .cartesian_product(0..grid[0].len())
        // for cell no on the loop and not on the right edge
        .filter(|coord @ (_, col)| !loop_coords.contains(coord) && *col != grid[0].len() - 1)
        // inside criteria
        .filter(|coord| is_inside(&grid, &loop_coords, *coord))
        .count() as u64)
}

// region:    --- Part 1

fn find_start(grid: &[&[u8]]) -> (usize, usize) {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|(row, col)| grid[*row][*col] == S)
        .expect("start should exits")
}

fn trace_loop(grid: &[&[u8]], start_coord: Coord) -> (u64, HashSet<Coord>) {
    let mut cur = start_coord;
    let mut steps = 0;
    let mut from = None;
    let mut loop_coords = HashSet::new();

    loop {
        let (new_cur, new_from) = walk(grid, cur, from);
        steps += 1;
        loop_coords.insert(cur);
        cur = new_cur;
        from = Some(new_from);
        if cur == start_coord {
            break;
        }
    }

    (steps, loop_coords)
}

// walk anticlockwise, up first
fn walk(grid: &[&[u8]], pos: Coord, from: Option<Direction>) -> ((usize, usize), Direction) {
    Direction::ALL
        .into_iter()
        // do not walk back to prev pos
        .filter(|dir| match from {
            None => true,
            Some(from) => *dir != from,
        })
        // dont walk off the grid
        .filter_map(|dir| {
            neighbour_coord_in_direction(dir, pos, grid.len(), grid[0].len())
                .map(|nbr_coord| (dir, nbr_coord))
        })
        // walk to connected cell with connected pipe
        .filter(|(dir, nbr_coord)| {
            connected(grid[pos.0][pos.1], grid[nbr_coord.0][nbr_coord.1], *dir)
        })
        .map(|(dir, nbr_coord)| (nbr_coord, dir.opposite()))
        .next()
        .expect("connected nbr exists")
}
// endregion: --- Part 1

// region:    --- Part 2

fn is_inside(grid: &[&[u8]], loop_coords: &HashSet<Coord>, (row, col): Coord) -> bool {
    // count all PIPE, F, L, 7, J on the right of grid[row][col]
    let mut counts = (col + 1..grid[0].len())
        .filter(|col| loop_coords.contains(&(row, *col)))
        .map(|col| grid[row][col])
        .counts();
    for pipe in [F, L, J, C7, PIPE] {
        counts.entry(pipe).or_insert(0);
    }

    let cross_count = (counts[&F] + counts[&L] + counts[&J] + counts[&C7]
        - std::cmp::min(counts[&L], counts[&J]) * 2
        - std::cmp::min(counts[&F], counts[&C7]) * 2)
        / 2;
    (counts[&PIPE] + cross_count) % 2 == 1
}

// endregion: --- Part 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {}

    #[test]
    fn test_find_start() {
        let grid: Vec<&[u8]> = include_bytes!("../test-1.txt")
            .split(|b| *b == NEWLINE)
            .collect();
        assert_eq!((1, 1), find_start(&grid));

        let grid: Vec<&[u8]> = include_bytes!("../test-2.txt")
            .split(|b| *b == NEWLINE)
            .collect();
        assert_eq!((2, 0), find_start(&grid));
    }
}
