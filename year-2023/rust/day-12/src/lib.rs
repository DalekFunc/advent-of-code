// region:    --- Modules
#![feature(slice_split_once)]
#![allow(unused)]
use std::{cell::OnceCell, collections::HashMap, sync::OnceLock};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
// endregion: --- Modules

pub fn part1(input: &str) -> Result<u64> {
    Ok(input
        .split("\n")
        .map(|line| possible_arrangement(line))
        .sum::<u64>())
}

pub fn part2(input: &str) -> Result<u64> {
    let lines: Vec<&str> = input.split("\n").collect();

    Ok(lines
        .iter()
        // .par_iter()
        .enumerate()
        .map(|(id, line)| {
            dbg!(id);
            possible_arrangement_5(line)
        })
        .sum::<u64>())
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
// static mut RESULT: OnceLock<HashMap<(u32, Vec<u32>), u64>> = OnceLock::new();
fn free_arrangement(free_space: u32, arrangement: &[u32]) -> u64 {
    // let map = unsafe { RESULT.get_or_init(|| HashMap::new()) };

    // match map.get(&(free_space, arrangement.to_vec())) {
    //     Some(result) => return *result,
    //     None => {
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
            let result = free_arrangement(
                free_space - arrangement[0] - {
                    if arrangement.len() == 1 {
                        0
                    } else {
                        1
                    }
                },
                &arrangement[1..],
            ) + free_arrangement(free_space - 1, arrangement);

            // let mut_map = unsafe { RESULT.get_mut().unwrap() };
            // mut_map
            //     .entry((free_space, arrangement.to_vec()))
            //     .or_insert(result);

            result
        // }
    // }
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
fn constrainted_arrangement(space: &[u8], arrangement: &[u32]) -> u64 {
    // short circuit if all spaces are free
    if space.iter().all(|b| *b == b'?') {
        return free_arrangement(space.len() as u32, arrangement);
    }

    if arrangement.len() == 0 {
        if space.iter().all(|&b| b != b'#') {
            return 1;
        } else {
            return 0;
        }
    }

    
    if space[0] == b'.' {
        constrainted_arrangement(&space[1..], arrangement)
    } else if space[0] == b'?' {
        // let (can_sub, sub_count) = can_substitute(space, arrangement[0] as usize);
        // // then we substitute the first cells with . or with #
        // constrainted_arrangement(&space[1..], arrangement)
        //     + if can_sub {
        //         constrainted_arrangement(&space[sub_count..], &arrangement[1..])
        //     } else {
        //         0
        //     }

        // we take as many ? spaces as we can until we reach . or one cell before #
        // then the combinations will be
        // max(?subspce, partial arrange * constrainted_arrangement(rest_space, rest_arrangement))
        
        let n = space.iter().take_while(|&&b| b == b'?').count();
        // // space cant be all ? otherwise it will be short circuiteed
        let free_space_count = if space[n] == b'#' { n - 1 } else { n };

        (0..arrangement.len())
            .map(|cut| {
                free_arrangement(free_space_count as u32, &arrangement[0..cut])
                    * constrainted_arrangement(&space[free_space_count..], &arrangement[cut..])
            })
            .max().unwrap()
    } else {
        let (can_sub, sub_count) = can_substitute(space, arrangement[0] as usize);
        // then we substitute the first cells with #
        if can_sub {
            constrainted_arrangement(&space[sub_count..], &arrangement[1..])
        } else {
            0
        }
    }
}

fn possible_arrangement(input: &str) -> u64 {
    let (_, (space, arrangement)) = parse_line(input).expect("parse correct");

    constrainted_arrangement(&space, &arrangement)
}

fn possible_arrangement_5(input: &str) -> u64 {
    let (_, (space, arrangement)) = parse_line(input).expect("parse correct");

    let mut new_space = Vec::new();
    new_space.extend_from_slice(&space);
    new_space.extend_from_slice(&[b'?']);
    new_space.extend_from_slice(&space);
    new_space.extend_from_slice(&[b'?']);
    new_space.extend_from_slice(&space);
    new_space.extend_from_slice(&[b'?']);
    new_space.extend_from_slice(&space);
    new_space.extend_from_slice(&[b'?']);
    new_space.extend_from_slice(&space);

    let mut new_arrangement = Vec::new();
    new_arrangement.extend_from_slice(&arrangement);
    new_arrangement.extend_from_slice(&arrangement);
    new_arrangement.extend_from_slice(&arrangement);
    new_arrangement.extend_from_slice(&arrangement);
    new_arrangement.extend_from_slice(&arrangement);

    constrainted_arrangement(&new_space, &new_arrangement)
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
    #[case(0,100,&[1, 1, 1, 1, 1,1, 1, 1, 1, 1, 1, 1, 1, 1])]

    fn test_free_arrangement(
        #[case] expected: u64,
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
    #[case(22150361247847371, "?????????????????????????????????????????????????????????????????????????????????????????? 2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1")]
    #[case(22150361247847371, "??????????????????????????????????????????????????????????????????????????????????????????. 2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1")]
    fn test_possible_arrangement(#[case] expected: u64, #[case] input: &str) {
        assert_eq!(expected, possible_arrangement(input));
    }
}
