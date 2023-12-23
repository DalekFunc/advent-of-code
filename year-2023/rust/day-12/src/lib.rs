// region:    --- Modules
#![allow(unstable_name_collisions)]
use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;
use parser::{parse_line, parse_line2};
use std::iter;
use token::Token;
// endregion: --- Modules

mod cacher;
mod parser;
mod token;

pub fn part1(input: &str) -> Result<u64> {
    let sum = input
        .split("\n")
        .map(|line| {
            let (_, (space, arrangement)) = parse_line(line).expect("parse correct");

            constrainted_arrangement(&space, &arrangement)
        })
        .sum::<u64>();

    Ok(sum)
}

pub fn part2(input: &str) -> Result<u64> {
    let sum = input
        .split("\n")
        .map(|line| {
            let (_, (tokens, seq)) = parse_line2(line).expect("parse correct");

            // expand space
            let tokens = iter::repeat(tokens)
                .take(5)
                .intersperse(vec![Token::Uncertain])
                .flatten()
                .collect_vec();
            let seq = iter::repeat(seq).take(5).flatten().collect_vec();

            combinations(tokens, seq)
        })
        .sum::<u64>();

    Ok(sum)
}

// region:    --- Part 1
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
            .max()
            .unwrap()
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
// endregion: --- Part 1

// region:    --- Part 2
#[cached]
fn combinations(tokens: Vec<Token>, seq: Vec<u8>) -> u64 {
    let (tokens, seq) = simplify(&tokens, &seq);

    let result = match (tokens.is_empty(), seq.is_empty()) {
        (true, true) => 1,
        (true, false) => 0,
        (false, true) => {
            if tokens.iter().all(|token| !token.is_block()) {
                1
            } else {
                0
            }
        }
        (false, false) => {
            if tokens_can_fit(&tokens, &seq) {
                // twisting
                match tokens.len() {
                    0 => unreachable!(),
                    1 => {
                        if seq == [1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => twist(tokens, seq),
                }
            } else {
                0
            }
        }
    };

    result
}

// ret: pos, value
fn first_max(seq: &[u8]) -> (usize, u8) {
    seq.iter().enumerate().fold(
        (0, 0),
        |record @ (_, max), (idx, elem)| {
            if *elem > max {
                (idx, *elem)
            } else {
                record
            }
        },
    )
}

// start pos of available slice
fn available_spaces(tokens: &[Token], length: usize) -> Vec<usize> {
    let mut positions = vec![];
    for pos in 0..tokens.len() - length + 1 {
        if tokens[pos..pos + length]
            .iter()
            .all(|token| !token.is_empty())
        {
            positions.push(pos);
        }
    }

    positions
}

fn twist<'a>(tokens: &'a [Token], seq: &'a [u8]) -> u64 {
    // get first max
    let (twist_at_seq, max) = first_max(seq);

    // find available spaces indexes in tokens
    let positions = available_spaces(tokens, max as usize);

    // allocate first max to available spaces
    // this way we only create 2 subproblems for each allocation
    positions
        .iter()
        .map(|&pos| {
            // make sure that before/behind max substitution, the token is not a block
            let before: Option<usize> = pos.checked_sub(1);
            let behind = if pos + (max as usize) < tokens.len() {
                Some(pos + max as usize)
            } else {
                None
            };
            let intermediate_result = if before.is_some() && tokens[before.unwrap()].is_block() {
                0
            } else if behind.is_some() && tokens[behind.unwrap()].is_block() {
                0
            } else {
                let pre = if let Some(before) = before {
                    tokens[..before].to_owned()
                } else {
                    tokens[..pos].to_owned()
                };

                let post = if let Some(behind) = behind {
                    tokens[behind + 1..].to_owned()
                } else {
                    tokens[pos + max as usize..].to_owned()
                };

                // go back to combinations for subproblems
                combinations(pre, seq[..twist_at_seq].to_owned())
                    * combinations(post, seq[twist_at_seq + 1..].to_owned())
            };

            intermediate_result
        })
        .sum::<u64>()
}

fn longest_substring_by(tokens: &[Token], predicate: impl Fn(&Token) -> bool) -> usize {
    tokens
        .iter()
        .chain(iter::once(&Token::Empty))
        .fold((false, 0, 0), |(_prev, acc, max), elem| {
            if predicate(elem) {
                (true, acc + 1, max)
            } else {
                (false, 0, std::cmp::max(acc, max))
            }
        })
        .2
}

fn longest_substring_of_block(tokens: &[Token]) -> usize {
    longest_substring_by(tokens, Token::is_block)
}

fn longest_substring_of_nonempty(tokens: &[Token]) -> usize {
    longest_substring_by(tokens, |token| !token.is_empty())
}

fn simplify<'a>(tokens: &'a [Token], seq: &'a [u8]) -> (&'a [Token], &'a [u8]) {
    let (tokens, seq) = simplify_front(tokens, seq);
    let (tokens, seq) = simplify_back(tokens, seq);
    (tokens, seq)
}

fn simplify_front<'a>(mut tokens: &'a [Token], mut seq: &'a [u8]) -> (&'a [Token], &'a [u8]) {
    loop {
        if tokens.is_empty() {
            return (tokens, seq);
        }

        if tokens.first().expect("first token").is_empty() {
            tokens = trimming_front(tokens)
        } else if can_match_front(tokens, seq) {
            (tokens, seq) = match_front(tokens, seq)
        } else {
            break;
        }
    }

    (tokens, seq)
}

fn simplify_back<'a>(mut tokens: &'a [Token], mut seq: &'a [u8]) -> (&'a [Token], &'a [u8]) {
    loop {
        if tokens.is_empty() {
            return (tokens, seq);
        }

        if tokens.last().expect("last token").is_empty() {
            tokens = trimming_back(tokens)
        } else if can_match_back(tokens, seq) {
            (tokens, seq) = match_back(tokens, seq)
        } else {
            break;
        }
    }

    (tokens, seq)
}

fn trimming_front(tokens: &[Token]) -> &[Token] {
    assert!(tokens.len() > 0);
    match tokens.iter().find_position(|token| !token.is_empty()) {
        Some((pos, _)) => &tokens[pos..],
        None => &[],
    }
}

fn trimming_back(tokens: &[Token]) -> &[Token] {
    assert!(tokens.len() > 0);
    match tokens.iter().rev().find_position(|token| !token.is_empty()) {
        Some((pos, _)) => &tokens[..tokens.len() - pos],
        None => &[],
    }
}

fn can_match_front<'a>(tokens: &'a [Token], seq: &'a [u8]) -> bool {
    // since we have done trimming before, the first character must be ? or #
    if tokens.is_empty() || tokens[0].is_uncertain() || seq.is_empty() {
        return false;
    }

    let match_len = seq[0] as usize;

    // #???#  3 ok
    // #?#.#  3 ok
    // #??#   3 no
    // ###.   ok
    // ###
    if tokens.len() == match_len {
        tokens
            .iter()
            .all(|token| token.is_block() || token.is_uncertain())
    } else {
        tokens.iter().take(match_len).all(|token| !token.is_empty())
            && tokens
                .iter()
                .skip(match_len)
                .next()
                .is_some_and(|elem| !elem.is_block())
    }
}

fn can_match_back<'a>(tokens: &'a [Token], seq: &'a [u8]) -> bool {
    // since we have done trimming before, the first character must be ? or #
    if tokens.is_empty() || tokens.last().unwrap().is_uncertain() || seq.is_empty() {
        return false;
    }

    let match_len = *seq.last().expect("last seq") as usize;

    if tokens.len() == match_len {
        tokens
            .iter()
            .all(|token| token.is_block() || token.is_uncertain())
    } else {
        tokens
            .iter()
            .rev()
            .take(match_len)
            .all(|token| !token.is_empty())
            && tokens
                .iter()
                .rev()
                .skip(match_len)
                .next()
                .is_some_and(|elem| !elem.is_block())
    }
}

fn match_front<'a>(mut tokens: &'a [Token], mut seq: &'a [u8]) -> (&'a [Token], &'a [u8]) {
    let match_len = *seq.first().expect("first seq") as usize;

    seq = if seq.len() > 1 { &seq[1..] } else { &[] };
    tokens = if tokens.len() <= match_len + 1 {
        &[]
    } else {
        &tokens[match_len + 1..]
    };

    (tokens, seq)
}

fn match_back<'a>(mut tokens: &'a [Token], mut seq: &'a [u8]) -> (&'a [Token], &'a [u8]) {
    let match_len = *seq.last().expect("last seq") as usize;

    seq = if seq.len() > 1 {
        &seq[..seq.len() - 1]
    } else {
        &[]
    };
    tokens = if tokens.len() <= match_len + 1 {
        &[]
    } else {
        &tokens[..tokens.len() - match_len - 1]
    };

    (tokens, seq)
}

// it is an incomplete guess
fn tokens_can_fit(tokens: &[Token], seq: &[u8]) -> bool {
    tokens.iter().filter(|t| !t.is_empty()).count()
        >= seq.iter().map(|&num| num as usize).sum::<usize>()
}
// endregion: --- Part 2

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use rstest_reuse::{self, *};

    use crate::parser::tokens;

    use super::*;

    #[rstest]
    // #[case(0, " 3,2,1")]
    // #[case(1, "???.### 1,1,3")]
    // #[case(0, ".??.### 1,1,3")]
    // #[case(1, "#??.### 1,1,3")]
    // #[case(1, "#?#.### 1,1,3")]
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
    fn test_combinations(#[case] expected: u64, #[case] fixture: &str) {
        let (_, (tokens, seq)) = parse_line2(fixture).expect("parse ok");

        assert_eq!(expected, combinations(tokens, seq))
    }

    #[rstest]
    #[case(3, "###....")]
    #[case(4, "###.####..#")]
    #[case(0, ".......")]
    #[case(1, "......#")]
    fn test_longest_substring_of_block(#[case] expected: usize, #[case] fixture: &str) {
        let (_, tokens) = tokens(fixture).expect("parse ok");

        assert_eq!(expected, longest_substring_of_block(&tokens))
    }

    #[rstest]
    #[case(3, "###....")]
    #[case(4, "###.####..#")]
    #[case(0, ".......")]
    #[case(1, "......#")]
    #[case(3, "#?#....")]
    #[case(4, "###.????..#")]
    #[case(0, ".......")]
    #[case(1, "...?...#")]
    fn test_longest_substring_of_non_empty(#[case] expected: usize, #[case] fixture: &str) {
        let (_, tokens) = tokens(fixture).expect("parse ok");

        assert_eq!(expected, longest_substring_of_nonempty(&tokens))
    }

    #[rstest]
    #[case("###.??????? 3,2,1")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case("??#?#?#?#?#?#?#? 1,3,1,6")]
    #[case("#??.### 1,1,3")]
    fn test_simplify_front(#[case] fixture: &str) {
        let (_, (tokens, seq)) = parse_line2(fixture).expect("parse ok");

        let trimmed_tokens = simplify_front(&tokens, &seq);
        println!("{:?}", trimmed_tokens);
    }

    #[rstest]
    #[case("###.??????? 3,2,1")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case("#??.### 1,1,3")]
    fn test_simplify_back(#[case] fixture: &str) {
        let (_, (tokens, seq)) = parse_line2(fixture).expect("parse ok");

        let trimmed_tokens = simplify_back(&tokens, &seq);
        println!("{:?}", trimmed_tokens);
    }

    #[rstest]
    #[case(".??..??...?##.?.??..??...?##. 1,1,3,1,1,3")]
    #[case("###.??????? 3,2,1")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case("#??.### 1,1,3")]
    #[case("...#...?????......#.... 1,3,1")]
    #[case("...#???????#......#.... 1,3,1")]
    fn test_my_simplify(#[case] fixture: &str) {
        use crate::token::print_tokens;

        let (_, (tokens, seq)) = parse_line2(fixture).expect("parse ok");

        let trimmed_tokens = simplify(&tokens, &seq);
        println!(
            "{}, {:?} -> {}, {:?}",
            print_tokens(&tokens),
            seq,
            print_tokens(trimmed_tokens.0),
            trimmed_tokens.1
        );
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

    #[template]
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
    #[case(22150361247847371, "?????????????????????????????????????????????????????????????????????????????????????????? 2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1")]
    #[case(22150361247847371, "??????????????????????????????????????????????????????????????????????????????????????????. 2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1")]
    fn sample_test_cases(#[case] expected: u64, #[case] input: &str) {}
}
