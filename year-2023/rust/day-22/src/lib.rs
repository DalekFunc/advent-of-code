// --- region: Modules
use crate::brick::{z_order, Brick, Orientation};
use anyhow::Result;
use parser::parse_file;
use std::collections::{HashMap, HashSet};

mod brick;
mod coordinates;
mod parser;
type BrickIndex = usize;
type Relations = HashMap<BrickIndex, HashSet<BrickIndex>>;
// --- endregion: Modules

pub fn part1(input: &str) -> Result<u64> {
    let (_, bricks) = parse_file(input).expect("parse ok");

    let (supporting, supported_by, _) = build_data_structures(bricks);

    let mut count = 0;
    for (_, bricks_above) in supporting {
        if bricks_above
            .iter()
            .all(|brick| supported_by[brick].len() > 1)
        {
            count += 1;
        }
    }

    Ok(count)
}

pub fn part2(input: &str) -> Result<u64> {
    let (_, bricks) = parse_file(input).expect("parse ok");
    let total = bricks.len();

    let (supporting, supported_by, lowest_z) = build_data_structures(bricks);

    let mut count = 0;
    for idx in 0..total {
        count += count_would_fall(&supporting, &supported_by, &lowest_z, idx);
    }

    Ok(count)
}

fn land_on(
    top_layers: &HashMap<(u32, u32), (BrickIndex, u32)>,
    brick: &Brick,
) -> (u32, Vec<BrickIndex>) {
    let blocks = if brick.orientation() == Orientation::Z {
        vec![brick.start]
    } else {
        brick.blocks()
    };

    let (land_on, supporters) =
        blocks
            .iter()
            .fold((0, vec![]), |(level, mut supporters), block| {
                if let Some((supporter, z)) = top_layers.get(&block.xy()) {
                    match z.cmp(&level) {
                        std::cmp::Ordering::Less => (level, supporters),
                        // maybe a new supporter
                        std::cmp::Ordering::Equal => (*z, {
                            supporters.push(*supporter);
                            supporters
                        }),
                        // a new supporter
                        std::cmp::Ordering::Greater => (*z, vec![*supporter]),
                    }
                } else {
                    (level, supporters)
                }
            });

    (land_on + 1, supporters)
}

fn build_data_structures(
    mut bricks: Vec<Brick>,
) -> (Relations, Relations, HashMap<BrickIndex, u32>) {
    bricks.sort_by(z_order);

    // record the current top most block foreach xy.
    let mut top_layers: HashMap<(u32, u32), (BrickIndex, u32)> = HashMap::new();
    let mut supporting: Relations = HashMap::new();
    let mut supported_by: Relations = HashMap::new();
    let mut lowest_z = HashMap::new();

    bricks.iter().enumerate().for_each(|(idx, brick)| {
        supporting.entry(idx).or_default();
        supported_by.entry(idx).or_default();

        let (z, supporters) = land_on(&top_layers, brick);

        // fill in supporter and supporting rel
        for supporter in supporters {
            supporting.entry(supporter).and_modify(|set| {
                set.insert(idx);
            });
            supported_by.entry(idx).and_modify(|set| {
                set.insert(supporter);
            });
        }

        // update top layers
        if brick.orientation() == Orientation::Z {
            top_layers
                .entry(brick.start.xy())
                .and_modify(|(brick_id, level)| {
                    *brick_id = idx;
                    *level = z + brick.length() - 1
                })
                .or_insert((idx, z + brick.length() - 1));
        } else {
            let blocks = brick.blocks();
            blocks.iter().for_each(|block| {
                top_layers
                    .entry(block.xy())
                    .and_modify(|(brick_id, level)| {
                        *brick_id = idx;
                        *level = z
                    })
                    .or_insert((idx, z));
            });
        }

        // update lowest_z
        lowest_z.insert(idx, z);
    });

    (supporting, supported_by, lowest_z)
}

fn would_fall(supported_by: &Relations, fall: &HashSet<BrickIndex>, target: BrickIndex) -> bool {
    supported_by[&target]
        .iter()
        .all(|supporter| fall.contains(supporter))
}

fn count_would_fall(
    supporting: &Relations,
    supported_by: &Relations,
    lowest_z: &HashMap<BrickIndex, u32>,
    target: BrickIndex,
) -> u64 {
    let mut count = 0;

    let mut descendants = Vec::from_iter(supporting[&target].iter().cloned());
    let mut next_generation = HashSet::new();
    let mut fall = HashSet::from([target]);
    loop {
        for de in descendants {
            if would_fall(supported_by, &fall, de) {
                fall.insert(de);
                count += 1;
                next_generation.extend(supporting[&de].iter());
            }
        }

        if next_generation.is_empty() {
            break;
        } else {
            let next_z = next_generation
                .iter()
                .map(|idx| lowest_z[idx])
                .min()
                .expect("min");

            let (next_des, next_gen): (Vec<_>, _) = next_generation
                .into_iter()
                .partition(|idx| lowest_z[idx] == next_z);

            descendants = next_des;
            next_generation = HashSet::from_iter(next_gen);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5)]
    fn part1_using_test_input_1(#[case] expected: u64) {
        let result = part1(include_str!("../test-1.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(7)]
    fn part2_using_test_input_1(#[case] expected: u64) {
        let result = part2(include_str!("../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
