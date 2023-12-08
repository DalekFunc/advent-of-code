use std::collections::HashMap;

use anyhow::Result;

use itertools::Itertools;

use crate::{math::factorize, parser::parse_document};

mod math;
mod parser;

pub fn part1(input: &str) -> Result<u64> {
    let (_, (instructions, left_map, right_map)) =
        parse_document(input).expect("instructions and directions");

    let mut instructions = instructions.chars().cycle();

    let mut pos = "AAA";
    let mut steps = 0;
    while pos != "ZZZ" {
        let Some(dir) = instructions.next() else {
            panic!("no instructions")
        };

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
    let (_, (instructions, left_map, right_map)) =
        parse_document(input).expect("instructions and directions");

    let instructions = instructions.chars().cycle();

    dbg!(left_map.keys().filter(|key| key.ends_with('A')).count());

    let starts = left_map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    let z_pos_appearances: Vec<_> = starts
        .iter()
        .map(|pos| {
            let z_steps = steps_end_on_z(pos, instructions.clone(), &left_map, &right_map, 1);
            z_steps[0]
        })
        .collect();
    // dbg!(&z_pos_appearances);

    let all_factors: Vec<_> = z_pos_appearances
        .iter()
        .map(|(steps, _)| factorize(*steps))
        .collect();
    // dbg!(&all_factors);

    let mut factors_counted = all_factors
        .into_iter()
        .flat_map(|factors| {
            factors
                .into_iter()
                .dedup_with_count()
                .collect::<Vec<(usize, u64)>>()
        })
        .collect::<Vec<(usize, u64)>>();

    factors_counted.sort_by(|lhs, rhs| {
        if lhs.1 == rhs.1 {
            lhs.0.cmp(&rhs.0)
        } else {
            lhs.1.cmp(&rhs.1)
        }
    });

    let mut factors = HashMap::new();
    for (new_count, factor) in factors_counted {
        factors
            .entry(factor)
            .and_modify(|count| {
                if new_count > *count {
                    *count = new_count
                }
            })
            .or_insert(new_count);
    }

    Ok(factors
        .into_iter()
        .map(|(factor, count)| factor.pow(count as u32))
        .product::<u64>())
}

fn steps_end_on_z<'a>(
    starting_pos: &'a str,
    instructions: impl Iterator<Item = char>,
    left_map: &HashMap<&'a str, &'a str>,
    right_map: &HashMap<&'a str, &'a str>,
    take: usize,
) -> Vec<(u64, &'a str)> {
    let mut steps = 0;
    let mut pos = starting_pos;

    instructions
        .filter_map(|dir| {
            if dir == 'L' {
                pos = left_map.get(pos).expect("entry in left map");
            } else {
                pos = right_map.get(pos).expect("entry in right map");
            }

            steps += 1;
            if pos.ends_with('Z') {
                Some((steps, pos))
            } else {
                None
            }
        })
        .take(take)
        .collect()
}
