#![feature(int_roundings)]

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::{
    cmp::{max, min},
    ops::{Div, Range},
};

pub fn part1(input: &str) -> Result<u64> {
    let ingreds: Vec<Vec<i64>> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    println!("{ingreds:?}");

    // narrow down acceptable ranges for each ingred.
    let mut ranges = [0..101, 0..101, 0..101, 0..101];
    for (left, right) in [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] {
        for idx in 0..4 {
            let (ll, rr) = get_range_limits(ingreds[left][idx], ingreds[right][idx], 100);
            println!("> {:?}, {:?}", ll, rr);
            ranges[left] = max(ranges[left].start, ll.start)..min(ranges[left].end, ll.end);
            ranges[right] = max(ranges[right].start, rr.start)..min(ranges[right].end, rr.end);
        }
    }

    println!("{:?}", ranges);

    // brute force
    let mut score = 0;
    for i in ranges[0].clone() {
        for j in ranges[1].start..ranges[1].end {
            if i + j >= 100 {
                break;
            }
            for k in ranges[2].start..ranges[2].end {
                if i + j + k >= 100 {
                    break;
                }
                let l = 100 - i - j - k;

                // println!("{} {} {} {}", i, j, k, l);
                let mut ingreds = ingreds.clone();
                [i, j, k, l].iter().enumerate().for_each(|(idx, quantity)| {
                    ingreds[idx].iter_mut().for_each(|value| {
                        *value *= quantity;
                    })
                });
                // dbg!(&ingreds);

                let sum: Vec<i64> = (0..4)
                    .map(|idx| {
                        let prop: i64 = ingreds.iter().map(|ingred| ingred[idx]).sum();
                        if prop.is_negative() {
                            0
                        } else {
                            prop
                        }
                    })
                    .collect();
                let new_score = sum.iter().product();

                score = max(score, new_score);
            }
        }
    }

    Ok(score as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    let ingreds: Vec<Vec<i64>> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    println!("{ingreds:?}");

    // narrow down acceptable ranges for each ingred.
    let mut ranges = [0..101, 0..101, 0..101, 0..101];
    for (left, right) in [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] {
        for idx in 0..4 {
            let (ll, rr) = get_range_limits(ingreds[left][idx], ingreds[right][idx], 100);
            println!("> {:?}, {:?}", ll, rr);
            ranges[left] = max(ranges[left].start, ll.start)..min(ranges[left].end, ll.end);
            ranges[right] = max(ranges[right].start, rr.start)..min(ranges[right].end, rr.end);
        }
    }

    println!("{:?}", ranges);

    // brute force
    let mut score = 0;
    // for i in ranges[0].clone() {
    //     for j in ranges[1].start..ranges[1].end {
    //         if i + j >= 100 {
    //             break;
    //         }
    //         for k in ranges[2].start..ranges[2].end {
    for i in 0..101 {
        for j in 0..101 {
            if i + j >= 100 {
                break;
            }
            for k in 0..101 {
                if i + j + k >= 100 {
                    break;
                }
                let l = 100 - i - j - k;

                let total_calorie: i64 = ingreds
                    .iter()
                    .zip([i, j, k, l])
                    .map(|(ingred, quantity)| ingred[4] * quantity)
                    .sum();

                if total_calorie != 500 {
                    continue;
                }

                let mut ingreds = ingreds.clone();
                [i, j, k, l].iter().enumerate().for_each(|(idx, quantity)| {
                    ingreds[idx].iter_mut().for_each(|value| {
                        *value *= quantity;
                    })
                });
                // dbg!(&ingreds);

                let sum: Vec<i64> = (0..4)
                    .map(|idx| {
                        let prop: i64 = ingreds.iter().map(|ingred| ingred[idx]).sum();
                        if prop.is_negative() {
                            0
                        } else {
                            prop
                        }
                    })
                    .collect();
                let new_score = sum.iter().product();

                if new_score > 0 {
                    println!("{} {} {} {} {}", i, j, k, l, new_score);
                }

                score = max(score, new_score);
            }
        }
    }

    Ok(score as u64)
}

// region:    --- Parser

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (_rest, data) = all_consuming(preceded(
        tuple((alpha1, tag(": "))),
        separated_list1(tag(", "), preceded(tuple((alpha1, space1)), complete::i64)),
    ))(input)?;

    dbg!(&_rest);
    dbg!(&data);

    Ok((_rest, data))
}

// endregion: --- Parser

// region:    --- Part 1
// L, R: -1, 2 -> 0..34, 67..101
fn get_range_limits(lhs: i64, rhs: i64, total: i64) -> (Range<i64>, Range<i64>) {
    if !lhs.is_positive() && !rhs.is_positive() {
        (0..total + 1, 0..total + 1) // if both are negative or zero, there are nothing we can do to make their sum positive
    } else if lhs > 0 && rhs > 0 {
        (0..total + 1, 0..total + 1)
    } else if lhs > 0 && rhs == 0 || lhs == 0 && rhs > 0 {
        if lhs.is_positive() {
            (1..total + 1, 0..total)
        } else {
            (0..total, 1..total + 1)
        }
    } else {
        let alpha = min(lhs, rhs).abs();
        let beta = max(lhs, rhs);
        let lower_bounded = (total * alpha).div(alpha + beta) + 1..total + 1;
        let upper_bounded = 0..(total * beta).div_ceil(alpha + beta);

        if lhs.is_negative() {
            (upper_bounded, lower_bounded)
        } else {
            (lower_bounded, upper_bounded)
        }
    }
}

// endregion: --- Part 1

#[cfg(test)]
mod tests {
    #![feature(int_roundings)]

    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        let input = include_str!("../test-1.txt");
        let ingreds: Vec<Vec<i64>> = input
            .lines()
            .map(|line| parse_line(line).unwrap().1)
            .collect();

        let mut ls = vec![];
        for idx in 0..4 {
            let (l, r) = get_range_limits(ingreds[0][idx], ingreds[1][idx], 100);
            println!("{:?}, {:?}", l, r);
            ls.push(l);
        }
        let l = ls
            .into_iter()
            .reduce(|lhs, rhs| max(lhs.start, rhs.start)..min(lhs.end, rhs.end));
        println!("{:?}", l);

        let mut l = 0..101;
        let mut r = 0..101;
        for idx in 0..4 {
            let (ll, rr) = get_range_limits(ingreds[0][idx], ingreds[1][idx], 100);
            println!("> {:?}, {:?}", ll, rr);
            l = max(l.start, ll.start)..min(l.end, ll.end);
            r = max(r.start, rr.start)..min(r.end, rr.end);
        }

        println!("{l:?}, {r:?}");
    }

    #[rstest]
    #[case("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8")]
    #[case("Candy: capacity 0, durability 5, flavor -1, texture 0, calories 8")]
    fn test_parse_line(#[case] input: &str) {
        parse_line(input);
    }

    #[rstest]
    #[case(-1, 2,100, (0..67, 34..101))]
    #[case(-1, 1,100, (0..50, 51..101))]
    #[case(1, -1,100, (51..101, 0..50))]
    #[case(1, -9,100, (91..101, 0..10))]
    #[case(100, 0,100, (1..101, 0..100))]
    #[case(0, 100,100,(0..100, 1..101))]
    #[case(4, -1, 100, (21..101, 0..80))]
    #[case(-4, 1, 100, (0..20, 81..101))]
    fn test_get_range_limits(
        #[case] lhs: i64,
        #[case] rhs: i64,
        #[case] total: i64,
        #[case] expected: (Range<i64>, Range<i64>),
    ) {
        assert_eq!(expected, get_range_limits(lhs, rhs, total));
    }
}
