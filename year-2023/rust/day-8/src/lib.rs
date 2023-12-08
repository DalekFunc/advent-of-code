use anyhow::{Result, anyhow};

use crate::parser::parse_document;

mod parser;

pub fn part1(input: &str) -> Result<u64> {
    let (_, (instructions, left_map, right_map)) = parse_document(input).expect("instructions and directions");

    let mut instructions = instructions.chars().cycle();

    let mut pos = "AAA";
    let mut steps = 0;
    while pos != "ZZZ" {
        let Some(dir) = instructions.next() else { panic!("no instructions") };

        if dir == 'L' {
            pos = left_map.get(pos).expect("entry in left map");
        } else {
            pos = right_map.get(pos).expect("entry in right map");
        }

        steps += 1;
    }

    Ok(steps)
}

pub fn part2(input: &str) -> Result<u64> {
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}