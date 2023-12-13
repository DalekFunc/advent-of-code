#![allow(unused)]

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{is_a, tag},
    multi::{many1, separated_list1},
    IResult,
};

pub fn part1(input: &[u8]) -> Result<u64> {
    let (_, maps) = parse_file(input).expect("parse file ok");

    Ok(maps
        .iter()
        .map(|m| {
            find_horizontal_mirror(m).unwrap_or(0) * 100 + find_vertical_mirror(m).unwrap_or(0)
        })
        .sum::<usize>() as u64)
}

pub fn part2(input: &[u8]) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

// region:    --- Parsing
type Map<'a> = Vec<&'a [u8]>;

fn map(input: &[u8]) -> IResult<&[u8], Map> {
    separated_list1(tag(b"\n"), is_a(".#"))(input)
}

fn parse_file(input: &[u8]) -> IResult<&[u8], Vec<Map>> {
    separated_list1(tag(b"\n\n"), map)(input)
}
// endregion: --- Parsing

// region:    --- Part 1

fn find_horizontal_mirror(map: &[&[u8]]) -> Option<usize> {
    for mirror in 1..map.len() {
        if mirror <= map.len() / 2 {
            if map[..mirror]
                .iter()
                .zip(map[mirror..mirror * 2].iter().rev())
                .all(|(x, y)| x == y)
            {
                return Some(mirror);
            }
        } else {
            if map[mirror * 2 - map.len()..mirror]
                .iter()
                .zip(map[mirror..].iter().rev())
                // .map(|v| {
                //     dbg!(std::str::from_utf8(v.0).unwrap());
                //     dbg!(std::str::from_utf8(v.1).unwrap());
                //     v
                // })
                .all(|(x, y)| x == y)
            {
                return Some(mirror);
            }
        }
    }

    None
}

fn rotate(input: &Map) -> Vec<Vec<u8>> {
    // let map = Vec::with_capacity(input[0].len());
    // for col in 0..input.len() {
    //     map.push(Vec::ne)
    // }

    let mut map = vec![vec![b'.'; input.len()]; input[0].len()];
    for (to_col, from_row) in (0..input.len()).rev().enumerate() {
        for idx in 0..input[0].len() {
            map[idx][to_col] = input[from_row][idx];
        }
    }

    map
}

fn find_vertical_mirror(map: &Map) -> Option<usize> {
    let map = rotate(map);
    let ref_map: Vec<&[u8]> = map.iter().map(|line| line.as_ref()).collect();

    find_horizontal_mirror(&ref_map)
}

// endregion: --- Part 1

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        let input = include_bytes!("../input.txt");

        let (_, maps) = parse_file(input).expect("parse ok");

        println!(
            "{:?}",
            maps[0]
                .iter()
                .map(|line| unsafe { std::str::from_utf8_unchecked(line) })
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn test_rotate() {
        let input = b"...
###
.#.";

        let (_, fixture) = map(input).expect("parse ok");

        println!(
            "{:?}",
            rotate(&fixture)
                .iter()
                .map(|line| unsafe { std::str::from_utf8_unchecked(line) })
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn test_horizontal_mirror() {
        let input = b"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let (_, fixture) = map(input).expect("parse ok");

        assert_eq!(Some(4), find_horizontal_mirror(&fixture));

        let input = b"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let (_, fixture) = map(input).expect("parse ok");

        assert_eq!(None, find_horizontal_mirror(&fixture));
    }

    #[test]
    fn test_vertical_mirror() {
        let input = b"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let (_, fixture) = map(input).expect("parse ok");

        assert_eq!(None, find_vertical_mirror(&fixture));

        let input = b"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let (_, fixture) = map(input).expect("parse ok");

        assert_eq!(Some(5), find_vertical_mirror(&fixture));
    }
}
