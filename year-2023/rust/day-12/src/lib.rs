#![feature(slice_split_once)]

use itertools::Itertools;
use std::hash;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take_till},
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn part1(input: &str) -> Result<u64> {
    Ok(input
        .split("\n")
        // .take(5)
        // .map(|line| dbg!(line))
        .map(|line| possible_arrangement(line))
        // .map(|v| dbg!(v))
        .sum::<u32>() as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

fn all_permutations_of_gears(input: &[u8]) -> Vec<Vec<u8>> {
    if input.is_empty() {
        vec![]
    } else if input.len() == 1 {
        match input[0] {
            b'.' => vec![vec![b'.']],
            b'#' => vec![vec![b'#']],
            b'?' => vec![vec![b'.'], vec![b'#']],
            _ => panic!("imp char"),
        }
    } else {
        match input[0] {
            b'.' => all_permutations_of_gears(&input[1..])
                .into_iter()
                .map(|mut bytes| {
                    bytes.insert(0, b'.');
                    bytes
                })
                .collect(),
            b'#' => all_permutations_of_gears(&input[1..])
                .into_iter()
                .map(|mut bytes| {
                    bytes.insert(0, b'#');
                    bytes
                })
                .collect(),
            b'?' => {
                let mut vec = all_permutations_of_gears(&input[1..])
                    .into_iter()
                    .map(|mut bytes| {
                        bytes.insert(0, b'.');
                        bytes
                    })
                    .collect::<Vec<_>>();

                vec.extend(
                    all_permutations_of_gears(&input[1..])
                        .into_iter()
                        .map(|mut bytes| {
                            bytes.insert(0, b'#');
                            bytes
                        })
                        .collect::<Vec<_>>(),
                );
                vec
            }
            _ => panic!("imp char"),
        }
    }
}

fn fulfill(input: &[u8], arrangement: &[u32]) -> bool {
    let counts: Vec<u32> = input
        .split(|&b| b == b'.')
        .filter(|bytes| bytes.len() > 0)
        .map(|bytes| bytes.len() as u32)
        .collect();

    counts.len() == arrangement.len() && counts.iter().zip(arrangement).all(|(lhs, rhs)| lhs == rhs)
}

// find how many possible combinations for the sequential arrangement in free space
// TODO: dynamic programming possible
fn free_arrangement(free_space: u32, arrangement: &[u32]) -> u32 {
    // early return for zero cases
    if arrangement.len() == 0 {
        return 1;
    }

    // works for free space is zero or > 0
    // early return for inadequate space
    if free_space < arrangement.iter().sum::<u32>() + arrangement.len() as u32 - 1 {
        return 0;
    }

    // We pick the first cell(s) to be either . or # and consume the arrangement
    // remember after a cell as need a . for separation
    // we recursively count the remaining possbility
    free_arrangement(
        free_space - arrangement[0] - {
            if arrangement.len() == 1 {
                0
            } else {
                1
            }
        },
        &arrangement[1..],
    ) + free_arrangement(free_space - 1, arrangement)

    // todo!();
}

// verify if the first n can be sub as #
fn can_substitute(space: &[u8], hash_len: usize) -> (bool, usize) {
    match space.len().cmp(&hash_len) {
        std::cmp::Ordering::Less => (false, 0),
        std::cmp::Ordering::Equal => {
            if space.iter().all(|&b| b != b'.') {
                (true, hash_len)
            } else {
                (false, 0)
            }
        }
        std::cmp::Ordering::Greater => {
            if space[0..hash_len].iter().all(|&b| b != b'.') && space[hash_len] != b'#' {
                (true, hash_len + 1)
            } else {
                (false, 0)
            }
        }
    }
}

// find combinations but this time we have some predefined . or #
fn constrainted_arrangement(space: &[u8], arrangement: &[u32]) -> u32 {
    // short circuit if all spaces are free
    if space.iter().all(|b| *b == b'?') {
        // println!(
        //     "All free: {}, {:?}",
        //     std::str::from_utf8(space).unwrap(),
        //     arrangement
        // );
        return free_arrangement(space.len() as u32, arrangement);
    }

    if arrangement.len() == 0 {
        // println!(
        //     "Some occupied: {}, {:?}, just 1 combo",
        //     std::str::from_utf8(space).unwrap(),
        //     arrangement
        // );
        if space.iter().all(|&b| b != b'#') {
            return 1;
        } else {
            return 0;
        }
    }

    if space[0] == b'.' {
        // println!(
        //     "handling . {} {:?}",
        //     std::str::from_utf8(space).unwrap(),
        //     arrangement
        // );
        constrainted_arrangement(&space[1..], arrangement)
    } else if space[0] == b'?' {
        // println!(
        //     "handling ? {} {:?}",
        //     std::str::from_utf8(space).unwrap(),
        //     arrangement
        // );

        let (can_sub, sub_count) = can_substitute(space, arrangement[0] as usize);
        // then we substitute the first cells with . or with #
        constrainted_arrangement(&space[1..], arrangement)
            + if can_sub {
                constrainted_arrangement(&space[sub_count..], &arrangement[1..])
            } else {
                0
            }
    } else {
        // println!(
        //     "handling # {} {:?}",
        //     std::str::from_utf8(space).unwrap(),
        //     arrangement
        // );

        let (can_sub, sub_count) = can_substitute(space, arrangement[0] as usize);
        // then we substitute the first cells with #
        if can_sub {
            constrainted_arrangement(&space[sub_count..], &arrangement[1..])
        } else {
            0
        }
    }
}

fn possible_arrangement(input: &str) -> u32 {
    let (_, (space, arrangement)) = parse_line(input).expect("parse correct");

    constrainted_arrangement(&space, &arrangement)
}

// region:    --- Parser
fn parse_line(input: &str) -> IResult<&str, (Vec<u8>, Vec<u32>)> {
    let (rest, (space, arrangement)) = separated_pair(
        take_till(|c| c == ' '),
        tag(" "),
        separated_list1(tag(","), complete::u32),
    )(input)?;

    Ok((rest, (space.as_bytes().to_owned(), arrangement)))
}
// endregion: --- Parser

#[cfg(test)]
mod tests {
    use itertools::all;
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        println!(
            "{:?}",
            all_permutations_of_gears(b"?.?")
                .iter()
                .map(|bytes| std::str::from_utf8(bytes).unwrap())
                .collect::<Vec<&str>>()
        );

        // println!(
        //     "{:?}",
        //     all_permutations_of_gears(b"???????")
        //         .iter()
        //         .map(|bytes| std::str::from_utf8(bytes).unwrap())
        //         .collect::<Vec<&str>>()
        // );
    }

    #[rstest]
    #[case(1,2, &[])]
    #[case(2,2,&[1])]
    #[case(1,2,&[2])]
    #[case(3,3,&[1])]
    #[case(2,3,&[2])]
    #[case(1,3,&[3])]
    #[case(1,3,&[1, 1])]
    #[case(0,3,&[1, 2])]
    fn test_free_arrangement(
        #[case] expected: u32,
        #[case] free_space: u32,
        #[case] arrangement: &[u32],
    ) {
        assert_eq!(expected, free_arrangement(free_space, arrangement));
    }

    #[rstest]
    #[case(true, b".#.##.###", &[1, 2,3])]
    #[case(false, b".##.##.###", &[1, 2,3])]
    #[case(false, b".######", &[1, 2,3])]
    fn test_fulfill(#[case] expected: bool, #[case] space: &[u8], #[case] arrangement: &[u32]) {
        assert_eq!(expected, fulfill(space, arrangement));
    }

    #[rstest]
    #[case((true, 3), b"???", 3)]
    #[case((true, 4), b"????", 3)]
    #[case((false, 0), b"??", 3)]
    #[case((true, 4), b"?##?", 3)]
    #[case((false, 0), b"???#", 3)]
    #[case((false, 0), b"####", 3)]
    #[case((false, 0), b"?###", 3)]
    #[case((false, 0), b".???", 3)]
    #[case((false, 0), b"??.?", 3)]
    #[case((true, 4), b"???.", 3)]
    #[case((true, 4), b"###?", 3)]
    #[case((true, 4), b"#?#?", 3)]
    fn test_can_substitute(
        #[case] expected: (bool, usize),
        #[case] space: &[u8],
        #[case] hash_len: usize,
    ) {
        assert_eq!(expected, can_substitute(space, hash_len))
    }

    #[rstest]
    // #[case(0, " 3,2,1")]
    // #[case(1, "???.### 1,1,3")]
    // #[case(0, ".??.### 1,1,3")]
    // #[case(1, "#??.### 1,1,3")]
    // #[case(1, "#?#.### 1,1,3")]
    // #[case(4, ".??..??...?##. 1,1,3")]
    // #[case(1, "?#?#?#?#?#?#?#? 1,3,1,6")]
    // #[case(1, "????.#...#... 4,1,1")]
    // #[case(4, "????.######..#####. 1,6,5")]
    // #[case(10, "?###???????? 3,2,1")]
    // #[case(10, ".###???????? 3,2,1")]
    // #[case(0, "####???????? 3,2,1")]
    // #[case(10, "###???????? 3,2,1")]
    // #[case(10, "###.??????? 3,2,1")]
    // #[case(10, "??????? 2,1")]
    #[case(3, "#??#??????? 7,1")]

    fn test_possible_arrangement_using_fulfill(#[case] expected: u32, #[case] input: &str) {
        let (_, (space, arrangement)) = parse_line(input).expect("parse correct");

        let all_perms = all_permutations_of_gears(&space);
        println!("{}", all_perms.len());

        let satisfied = all_perms
            .iter()
            // .map(|v| {
            //     println!("{:?}", std::str::from_utf8(v).unwrap());
            //     v
            // })
            .filter(|s| fulfill(s, &arrangement))
            .map(|v| {
                println!("{:?}", std::str::from_utf8(v).unwrap());
                v
            })
            .count();
        assert_eq!(expected, satisfied as u32);
    }

    #[rstest]
    #[case(0, " 3,2,1")]
    #[case(1, "???.### 1,1,3")]
    #[case(0, ".??.### 1,1,3")]
    #[case(1, "#??.### 1,1,3")]
    #[case(1, "#?#.### 1,1,3")]
    #[case(4, ".??..??...?##. 1,1,3")]
    #[case(1, "?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case(1, "????.#...#... 4,1,1")]
    #[case(4, "????.######..#####. 1,6,5")]
    #[case(10, "?###???????? 3,2,1")]
    #[case(10, ".###???????? 3,2,1")]
    #[case(0, "####???????? 3,2,1")]
    #[case(10, "###???????? 3,2,1")]
    #[case(10, "###.??????? 3,2,1")]
    #[case(10, "??????? 2,1")]
    fn test_possible_arrangement(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, possible_arrangement(input));
    }
}
