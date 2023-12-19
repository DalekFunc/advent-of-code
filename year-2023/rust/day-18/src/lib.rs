use std::collections::{BTreeMap, HashMap, VecDeque};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{self, newline, one_of, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub fn part1(input: &str) -> Result<u64> {
    let size = 801;

    let mut field = vec![vec![0; size]; size];

    let mut pos = (size / 2, size / 2);

    let (_, instructions) = parse_file(input).expect("parse ok");

    instructions.iter().for_each(|inst| {
        match inst.0 {
            Direction::Up => {
                for row in pos.0 - inst.1 as usize..pos.0 {
                    field[row][pos.1] = 1;
                }
                pos.0 = pos.0 - inst.1 as usize;
            }
            Direction::Down => {
                for row in pos.0 + 1..pos.0 + 1 + inst.1 as usize {
                    field[row][pos.1] = 1;
                }
                pos.0 = pos.0 + inst.1 as usize;
            }
            Direction::Right => {
                // dug
                field[pos.0][pos.1 + 1..pos.1 + 1 + inst.1 as usize].fill(1);
                // move pos
                pos.1 = pos.1 + inst.1 as usize;
            }
            Direction::Left => {
                // dug
                field[pos.0][pos.1 - inst.1 as usize..pos.1].fill(1);
                // move pos
                pos.1 = pos.1 - inst.1 as usize;
            }
        }
    });

    // floodfill
    let mut to_fill = VecDeque::new();

    // find first 0 after first 1 appearance
    let mut one_appeared = 0;
    let mut fill_target = (0, 0);
    for row in 0..size {
        if field[row][size / 2] == 1 {
            one_appeared += 1;
        }
        if one_appeared >= 1 && field[row][size / 2] == 0 {
            fill_target = (row, size / 2);
            break;
        }
    }

    to_fill.push_back(fill_target);

    while let Some(pos) = to_fill.pop_front() {
        let nbrs = neighbours(pos, size);

        for nbr in nbrs {
            if field[nbr.0][nbr.1] == 1 {
                continue;
            } else {
                field[nbr.0][nbr.1] = 1;
                to_fill.push_back(nbr);
            }
        }
    }

    let matrix = Matrix(field.clone());
    println!("{}", matrix);

    // count
    let cubic = field
        .iter()
        .map(|row| row.iter().filter(|land| **land == 1).count())
        .sum::<usize>();

    Ok(cubic as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    // let size = 801;
    // let mut field = vec![vec![0; size]; size];
    // let mut pos = (size / 2, size / 2);
    let (_, instructions) = parse_file2(input).expect("parse ok");

    let mut vertices: Vec<(i64, i64)> = vec![(0, 0)];
    let mut pos = (0, 0);
    instructions.iter().for_each(|inst| {
        match inst.0 {
            Direction::Up => {
                pos.0 = pos.0 - inst.1 as i64;
            }
            Direction::Down => {
                pos.0 = pos.0 + inst.1 as i64;
            }
            Direction::Right => {
                pos.1 = pos.1 + inst.1 as i64;
            }
            Direction::Left => {
                pos.1 = pos.1 - inst.1 as i64;
            }
        }
        vertices.push(pos);
    });

    assert_eq!(vertices[0], *vertices.last().unwrap());

    // let vertices = vec![(0,0), (0, 2), (2, 2), (2, 0), (0, 0)];
    // normalize
    // let min = vertices.iter().fold((i64::MAX, i64::MAX), |(min_x, min_y), elem| {
    //     (
    //         std::cmp::min(min_x, elem.0),
    //         std::cmp::min(min_y, elem.1)
    //     )
    // });
    // dbg!(min);

    // vertices.iter_mut().for_each(|elem| {
    //     elem.0 -= min.0;
    //     elem.1 -= min.1;
    // });

    // let mut xs = vertices.iter().map(|elem| elem.0 ).collect_vec();
    // xs.sort();
    // xs.dedup();
    // let x_inc = BTreeMap::from_iter(xs.into_iter().enumerate().map(|(a, b)|(b, a)));
    // dbg!(&x_inc);
    // let mut ys = vertices.iter().map(|elem| elem.1 ).collect_vec();
    // ys.sort();
    // ys.dedup();
    // let y_inc = BTreeMap::from_iter(ys.into_iter().enumerate().map(|(a, b)|(b, a)));
    // dbg!(&y_inc);

    // vertices.iter_mut().for_each(|elem| {
    //     elem.0 += x_inc[&elem.0] as i64;
    //     elem.1 += y_inc[&elem.1] as i64;
    // });

    let mut inner_area: i64 = vertices
        .iter()
        .tuple_windows()
        .map(|(l, r)| l.0 * r.1 - r.0 * l.1)
        .sum();

    inner_area = inner_area / 2;
    inner_area = inner_area.abs();

    let edges: i64 = vertices
        .iter()
        .tuple_windows()
        .map(|(l, r)| {
            let vertical = r.0 - l.0;
            let horizontal = r.1 - l.1;

            let len = (vertical > 0).then(|| vertical).unwrap_or(0)
                + (horizontal < 0).then(|| horizontal.abs()).unwrap_or(0);

            len
        })
        .sum();

    dbg!(&instructions);

    let corner_turn_up_left = instructions
        .iter()
        .tuple_windows()
        .filter(|(l, r)| l.0 == Direction::Up && r.0 == Direction::Left)
        .count();

    let corner_turn_left_up = instructions
    .iter()
    .tuple_windows()
    .filter(|(l, r)| l.0 == Direction::Left && r.0 == Direction::Up)
    .count();

    dbg!(edges);
    dbg!(corner_turn_up_left);
    dbg!(corner_turn_left_up);

    // sum.abs();
    Ok((inner_area + edges - corner_turn_up_left as i64 + corner_turn_left_up as i64) as u64)
    // Ok((inner_area) as u64)
}

fn neighbours(pos: Coord, max: usize) -> Vec<Coord> {
    let up = (pos.0 != 0).then(|| (pos.0 - 1, pos.1));
    let down = (pos.0 != max - 1).then(|| (pos.0 + 1, pos.1));
    let left = (pos.1 != 0).then(|| (pos.0, pos.1 - 1));
    let right = (pos.1 != max - 1).then(|| (pos.0, pos.1 + 1));

    let nbrs = vec![up, down, left, right];
    nbrs.into_iter()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> std::fmt::Display for Matrix<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.0 {
            for elem in line {
                write!(f, "{}", elem);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

type DugInstruction = (Direction, u8, Color);
type DugInstruction2 = (Direction, u32);

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, (red, green, blue)) = delimited(
        tag("("),
        preceded(tag("#"), tuple((hex_primary, hex_primary, hex_primary))),
        tag(")"),
    )(input)?;

    Ok((input, Color { red, green, blue }))
}

fn dug_instruction(input: &str) -> IResult<&str, DugInstruction> {
    let (input, raw_dir) = terminated(one_of("UDLR"), space1)(input)?;
    let (input, len) = terminated(complete::u8, space1)(input)?;
    let (rest, color) = hex_color(input)?;

    Ok((rest, (Direction::from(raw_dir), len, color)))
}

fn parse_file(input: &str) -> IResult<&str, Vec<DugInstruction>> {
    separated_list1(newline, dug_instruction)(input)
}

fn dug_instruction2(input: &str) -> IResult<&str, DugInstruction2> {
    let (rest, (len_raw, dir_raw)) = preceded(
        take_until("#"),
        delimited(
            tag("#"),
            tuple((take_while_m_n(5, 5, is_hex_digit), one_of("0123"))),
            tag(")"),
        ),
    )(input)?;

    let dir = match dir_raw {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("invalid char"),
    };
    let len = u32::from_str_radix(len_raw, 16).expect("u32 conv ok");

    Ok((rest, (dir, len)))
}

fn parse_file2(input: &str) -> IResult<&str, Vec<DugInstruction2>> {
    separated_list1(newline, dug_instruction2)(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[test]
    fn test_parsing() {
        let fixture = "R 6 (#70c710)";
        let dug = dug_instruction(fixture);

        println!("{:?}", dug);
        assert!(dug.is_ok());
    }

    #[test]
    fn test_parsing2() {
        let fixture = "R 6 (#70c710)";
        let dug = dug_instruction2(fixture);

        println!("{:?}", dug);
        assert!(dug.is_ok());
    }
}
