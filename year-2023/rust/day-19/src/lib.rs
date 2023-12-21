// --- Modules
#![allow(unused)]
#![allow(dead_code)]
use std::collections::VecDeque;

use anyhow::{anyhow, Result};
use parser::parse_file;
use range::{PartRange, Range};
use types::{Part, Workflow};

mod parser;
mod range;
mod types;
// --- Modules

pub fn part1(input: &str) -> Result<u64> {
    let (_, (workflows, parts)) = parse_file(input).expect("parse ok");

    let mut accepted = vec![];

    for part in parts {
        let mut name = "in";

        'outer: loop {
            match name {
                "A" => {
                    accepted.push(part);
                    break;
                }
                "R" => break,
                _ => {
                    for rule in &workflows[name].rules {
                        if let Some(new_name) = rule.satisfy(part) {
                            name = new_name;
                            continue 'outer;
                        }
                    }
                    name = workflows[name].catch_all;
                }
            }
        }
    }

    let total_rating = accepted.iter().map(Part::rating).sum::<u32>();

    Ok(total_rating as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    let (_, (workflows, _)) = parse_file(input).expect("parse ok");

    let mut accepted = vec![];
    let full_range = PartRange {
        x: Range {
            start: 1,
            end: 4001,
        },
        m: Range {
            start: 1,
            end: 4001,
        },
        a: Range {
            start: 1,
            end: 4001,
        },
        s: Range {
            start: 1,
            end: 4001,
        },
    };
    let mut ranges = VecDeque::from([(full_range, "in")]);

    while let Some((mut range, target)) = ranges.pop_front() {
        // dbg!(&range);
        // dbg!(&target);
        match target {
            "A" => {
                accepted.push(range);
            }
            "R" => {}
            _ => {
                let mut range = Some(range);
                // since each rule only compare one
                // each rule should bisect or not interact with the existing range
                for rule in &workflows[target].rules {
                    // dbg!(&rule);
                    let (satisfied, rest) = rule.satisfy_range(&range.expect("range exist"));
                    if let Some(satisfied_range) = satisfied {
                        ranges.push_back((satisfied_range, rule.target));
                        // dbg!(&ranges);
                    }
                    range = rest;
                    if range.is_none() {
                        break;
                    }
                }

                //push remaining range if any
                if let Some(range) = range {
                    ranges.push_back((range, workflows[target].catch_all));
                };
            }
        }
    }

    let combinations = accepted.iter().map(PartRange::combinations).sum::<u64>();

    // intuition
    // back track from workflow which accept to "in"
    // find all acceptable ranges
    // count the ranges combintaitons
    // be careful of the overlaps.

    Ok(combinations)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}
