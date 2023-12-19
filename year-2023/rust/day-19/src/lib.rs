// -- Modules
#![allow(unused)]
#![allow(dead_code)]
use anyhow::{anyhow, Result};
use parser::parse_file;
use types::{Part, Workflow};

mod parser;
mod types;
// -- Modules

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
    Err(anyhow!("Not Implemented."))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}
