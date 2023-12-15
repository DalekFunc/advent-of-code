#![allow(unused)]

use anyhow::{Result, anyhow};
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use itertools::Itertools;

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

pub fn part2(input: &str) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

// region:    --- Parsing
type Map<'a> = Vec<&'a [u8]>;

fn parse_map(input: &[u8]) -> IResult<&[u8], Map> {
    separated_list1(tag(b"\n"), is_a(".O#"))(input)
}

// endregion: --- Parsing

#[derive(Debug, Clone, Copy, PartialEq, Eq,PartialOrd, Ord)]
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

struct Grid(Vec<Vec<Rock>>);

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