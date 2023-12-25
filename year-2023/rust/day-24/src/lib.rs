#![allow(unused)]
use anyhow::{anyhow, Result};
use itertools::Itertools;
use parser::{parse_file, HailStone, Vector2D};

mod parser;

// const WINDOW_START: f64 = 7.0;
// const WINDOW_END: f64 = 27.0;

// const WINDOW_START: f64 = 0.0;
// const WINDOW_END: f64 = 20.0;

// const WINDOW_START: f64 = 0.0;
// const WINDOW_END: f64 = 200000000000000.0;
const WINDOW_START: f64 = 200000000000000.0;
const WINDOW_END: f64 = 400000000000000.0;

pub fn part1(input: &str) -> Result<u64> {
    let (_, stones) = parse_file(input).expect("parse ok");

    let mut count = 0;
    for left in 0..stones.len() {
        for right in left + 1..stones.len() {
            let a = &stones[left];
            let b = &stones[right];

            let determinant = collide(a, b);
            // dbg!(a, b, &determinant);
            dbg!(&determinant);
            let observed = if let CollideScenario::IntersectAt { position } = determinant {
                position.x <= WINDOW_END
                    && position.x >= WINDOW_START
                    && position.y <= WINDOW_END
                    && position.y >= WINDOW_START
            } else {
                false
            };
            if dbg!(observed) {
                count += 1;
            }
        }
    }

    Ok(count as u64)
}

#[derive(Debug)]
enum CollideScenario {
    Parallel,
    IntersectAt { position: Vector2D },
    OriginateFrom { position: Vector2D },
}

fn collide(a: &HailStone, b: &HailStone) -> CollideScenario {
    if a.spd.angle() == b.spd.angle() {
        CollideScenario::Parallel
    } else {
        // y = m1x + c1
        let m1 = a.spd.y / a.spd.x;
        let c1 = a.pos.y - a.pos.x / a.spd.x * a.spd.y;

        let m2 = b.spd.y / b.spd.x;
        let c2 = b.pos.y - b.pos.x / b.spd.x * b.spd.y;

        let cx = (c2 - c1) / (m1 - m2);
        let cy = m1 * cx + c1;

        let t1 = (cx - a.pos.x) / a.spd.x;
        let t2 = (cx - b.pos.x) / b.spd.x;

        if t1 < 0.0 || t2 < 0.0 {
            CollideScenario::OriginateFrom {
                position: Vector2D { x: cx, y: cy },
            }
        } else {
            CollideScenario::IntersectAt {
                position: Vector2D { x: cx, y: cy },
            }
        }
    }
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

    #[rstest]
    #[case(2)]
    fn part1_using_test_input_1(#[case] expected: u64) {
        let result = part1(include_str!("../test-1.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(0)]
    fn part2_using_test_input_1(#[case] expected: u64) {
        let result = part2(include_str!("../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
