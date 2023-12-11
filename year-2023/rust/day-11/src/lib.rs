use std::cmp::{max, min};

use anyhow::Result;

pub fn part1(input: &[u8]) -> Result<u64> {
    let mut universe = Galaxy::new(input.split(|b| *b == b'\n').collect());

    universe.expand();

    let galaxies: Vec<Coord> = universe.find_galaxies();

    let mut sum = 0;
    for g1 in 0..galaxies.len() {
        for g2 in g1 + 1..galaxies.len() {
            sum += universe.distance_between(galaxies[g1], galaxies[g2], 1);
        }
    }

    Ok(sum)
}

pub fn part2(input: &[u8], scale_factor: u64) -> Result<u64> {
    let mut universe = Galaxy::new(input.split(|b| *b == b'\n').collect());

    universe.expand();

    let galaxies: Vec<Coord> = universe.find_galaxies();

    let mut sum = 0;
    for g1 in 0..galaxies.len() {
        for g2 in g1 + 1..galaxies.len() {
            sum += universe.distance_between(galaxies[g1], galaxies[g2], scale_factor - 1);
        }
    }

    Ok(sum)
}

type Coord = (usize, usize);

fn manhatten_distance(c1: Coord, c2: Coord) -> u64 {
    ((c2.0 as isize - c1.0 as isize).abs() + (c2.1 as isize - c1.1 as isize).abs()) as u64
}

#[derive(Debug)]
struct Galaxy<'a> {
    grid: Vec<&'a [u8]>,
    rows_expanded: Vec<usize>,
    cols_expanded: Vec<usize>,
}

impl<'a> Galaxy<'a> {
    fn new(grid: Vec<&'a [u8]>) -> Self {
        Self {
            grid,
            rows_expanded: Vec::new(),
            cols_expanded: Vec::new(),
        }
    }

    fn expand(&mut self) {
        // expand row
        self.rows_expanded = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(id, content)| {
                if content.iter().all(|c| *c == b'.') {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();

        // expand col
        self.cols_expanded = (0..self.grid[0].len())
            .filter(|col| {
                (0..self.grid.len())
                    .map(|row| self.grid[row][*col])
                    .all(|c| c == b'.')
            })
            .collect();
    }

    fn find_galaxies(&self) -> Vec<Coord> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(row, content)| {
                content
                    .iter()
                    .enumerate()
                    .filter_map(|(col, cell)| {
                        if *cell == b'#' {
                            Some((row, col))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Coord>>()
            })
            .collect()
    }

    fn distance_between(&self, galaxy1: Coord, galaxy2: Coord, scale: u64) -> u64 {
        manhatten_distance(galaxy1, galaxy2)
            + (min(galaxy1.0, galaxy2.0) + 1..max(galaxy1.0, galaxy2.0))
                .filter(|row| self.rows_expanded.contains(row))
                .count() as u64
                * scale
            + (min(galaxy1.1, galaxy2.1) + 1..max(galaxy1.1, galaxy2.1))
                .filter(|col| self.cols_expanded.contains(col))
                .count() as u64
                * scale
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(0,  (0, 0), (0, 0))]
    #[case(1, (0, 0), (0, 1))]
    #[case(1, (0, 0), (1, 0))]
    #[case(2, (0, 0), (1, 1))]
    #[case(2,  (1, 1), (0, 0))]
    #[case(5, (0, 0), (2, 3))]
    fn test_manhatten_distance(#[case] expected: u64, #[case] c1: Coord, #[case] c2: Coord) {
        assert_eq!(expected, manhatten_distance(c1, c2));
    }

    #[rstest]
    #[case(false, 0)]
    #[case(false, 1)]
    #[case(true, 3)]
    fn test_expand_row(#[case] expected: bool, #[case] row: usize) {
        let mut universe = Galaxy::new(
            include_bytes!("../test-1.txt")
                .split(|b| *b == b'\n')
                .collect(),
        );
        universe.expand();

        assert_eq!(expected, universe.rows_expanded.contains(&row))
    }

    #[rstest]
    #[case(false, 0)]
    #[case(false, 1)]
    #[case(true, 2)]
    fn test_expand_col(#[case] expected: bool, #[case] col: usize) {
        let mut universe = Galaxy::new(
            include_bytes!("../test-1.txt")
                .split(|b| *b == b'\n')
                .collect(),
        );
        universe.expand();

        assert_eq!(expected, universe.cols_expanded.contains(&col))
    }

    #[rstest]
    #[case(15, (0, 3), (8, 7))]
    #[case(9, (5, 1), (9, 4))]
    fn test_distance_after_expansion(
        #[case] expected: u64,
        #[case] galaxy1: Coord,
        #[case] galaxy2: Coord,
    ) {
        let mut universe = Galaxy::new(
            include_bytes!("../test-1.txt")
                .split(|b| *b == b'\n')
                .collect(),
        );
        universe.expand();

        assert_eq!(expected, universe.distance_between(galaxy1, galaxy2, 1))
    }

    #[rstest]
    #[case(1030, 10)]
    #[case(8410, 100)]
    fn test_distance_after_expansion_scaled(#[case] expected: u64, #[case] scale: u64) {
        assert_eq!(
            expected,
            part2(include_bytes!("../test-1.txt"), scale).unwrap()
        )
    }
}
