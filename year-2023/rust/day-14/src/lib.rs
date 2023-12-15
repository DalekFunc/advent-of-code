#![allow(unused)]

use anyhow::{Result, anyhow};
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use itertools::Itertools;
use std::collections::HashMap;
pub fn part1(input: &[u8]) -> Result<u64> {
    let (_, input) = parse_map(input).expect("parse ok");

    let mut grid = Grid(vec![vec![Rock::Empty; input.len() + 1]; input[0].len()]);

    for row in 0..input[0].len() {
        grid.0[row][input.len()] = Rock::Cube;
    }
    for (to_col, from_row) in (0..input.len()).rev().enumerate() {
        for idx in 0..input[0].len() {
            grid.0[idx][to_col] = input[from_row][idx].into();
        }
    }

    let mut score = 0;
    // println!("{}", grid);
    for row in 0..grid.0.len() {
        let mut positions = grid.0[row].iter().positions(|&rock| rock == Rock::Cube).collect_vec();


        grid.0[row][0..positions[0]].sort();
        for (&from, &to) in positions.iter().tuple_windows() {
            if from + 1 < grid.0[0].len() {
                grid.0[row][from + 1..to].sort();
            }
        }

        score += grid.0[row].iter().fold((0, 1), |(acc, point), &elem|{ if elem == Rock::Round {
            (acc + point, point + 1)
        } else {
            (acc, point + 1)
        }}).0;
    }
    // println!("{}", grid);
    // println!("{}", score);


    Ok(score)
}

pub fn part2(input: &[u8]) -> Result<u64> {
    let (_, input) = parse_map(input).expect("parse ok");

    let mut grid = Grid(vec![vec![Rock::Empty; input.len() + 2]; input[0].len() + 2]);
    let grid_width = input.len() + 2;
    let grid_height = input[0].len() + 2;

    // fill first and last column with Cube
    for row in 0..grid_height {
        grid.0[row][grid_width-1] = Rock::Cube;
    }
    for row in 0..grid_height {
        grid.0[row][0] = Rock::Cube;
    }
    // fill first and last row with Cube
    grid.0[0].fill(Rock::Cube);
    grid.0[grid_height - 1].fill(Rock::Cube);

    for (to_col, from_row) in (0..input.len()).rev().enumerate() {
        for idx in 0..input[0].len() {
            grid.0[idx+1][to_col+1] = input[from_row][idx].into();
        }
    }
    // println!("{}", grid);

    let mut patterns = HashMap::new();
    let mut cycle = 0;
    let mut cycle_end = 0;
    for cycle_count in 1..=1000000000 {
        // dbg!(cycle_count);
        grid.cycle();

        // let summary = grid.summary();

        if patterns.contains_key(&grid) {
            cycle_end = cycle_count;
            cycle = cycle_count - patterns.get(&grid).expect("pattern exists");
            break;
        } else {
            patterns.insert(grid.clone(), cycle_count);
        }
        // println!("{}", grid);

    }
    dbg!(cycle_end);
    //
    dbg!(cycle);
    let cycle_begin = cycle_end - cycle;
    dbg!(cycle_begin + (1000000000 - cycle_end) % cycle);

    let end_grid = patterns.iter().filter(|(grid, c)| **c == cycle_begin + (1000000000 - cycle_end) % cycle).exactly_one().expect("!").0;

    Ok(end_grid.score())
}

// region:    --- Parsing
type Map<'a> = Vec<&'a [u8]>;

fn parse_map(input: &[u8]) -> IResult<&[u8], Map> {
    separated_list1(tag(b"\n"), is_a(".O#"))(input)
}

// endregion: --- Parsing

#[derive(Debug, Clone, Copy, PartialEq, Eq,PartialOrd, Ord, Hash)]
enum Rock {
    Empty,
    Round,
    Cube,
}

impl From<u8> for Rock {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Rock::Empty,
            b'O' => Rock::Round,
            b'#' => Rock::Cube,
            _ => panic!("unexpected character"),
        }
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {

        write!(f, "{}", match self {
            Rock::Cube => "#",
            Rock::Round => "O",
            Rock::Empty => "."
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid(Vec<Vec<Rock>>);

impl Grid {
    fn rotate(&mut self) {
        let mut matrix = vec![vec![Rock::Empty; self.0.len()]; self.0[0].len()];

        for (to_col, from_row) in (0..self.0.len()).rev().enumerate() {
            for idx in 0..self.0[0].len() {
                matrix[idx][to_col] = self.0[from_row][idx].into();
            }
        }

        self.0 = matrix;
    }

    fn rolling(&mut self) {
        for row in 0..self.0.len() {
            let mut positions = self.0[row].iter().positions(|&rock| rock == Rock::Cube).collect_vec();


            self.0[row][0..positions[0]].sort();
            for (&from, &to) in positions.iter().tuple_windows() {
                if from + 1 < self.0[0].len() {
                    self.0[row][from + 1..to].sort();
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.rolling();
        self.rotate(); // west
        self.rolling();
        self.rotate(); // south
        self.rolling();
        self.rotate(); //east
        self.rolling();
        self.rotate(); // back to north
    }

    // for part 2 only
    fn score(&self) -> u64 {
        let mut score = 0;
        for row in 0..self.0.len() {
            score += self.0[row].iter().fold((0, 0), |(acc, point), &elem|{ if elem == Rock::Round {
                (acc + point, point + 1)
            } else {
                (acc, point + 1)
            }}).0;
        }
        score
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.0 {
            for rock in line {
                write!(f, "{}", rock);
            }
            write!(f, "\n");
        }
        Ok(())
    }
    }

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}