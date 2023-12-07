use anyhow::{anyhow, Result};
use disjoint_range::DisjointRange;
use parser::full;

pub mod disjoint_range;
pub mod parser;
pub mod range_map;

pub fn part1(input: &str) -> Result<u64> {
    let (_, (seeds, listings)) = full(input).expect("Parse successful.");

    let condensed_map = listings
        .into_iter()
        .map(|idmap| idmap.mappings)
        .reduce(|lhs, rhs| lhs.concatenate(&rhs))
        .expect("Condense map should not fail.");

    Ok(seeds
        .into_iter()
        .map(|seed| condensed_map.map(seed))
        .min()
        .unwrap())
}

pub fn part2(input: &str) -> Result<u64> {
    let (_, (seeds, listings)) = full(input).expect("Parse successful.");

    let condensed_map = listings
        .into_iter()
        .map(|idmap| idmap.mappings)
        .reduce(|lhs, rhs| lhs.concatenate(&rhs))
        .expect("Condense map should not fail.");

    // expand seeds into ranges
    let mut seed_ranges: Vec<DisjointRange> = seeds
        .chunks(2)
        .map(|pair| (pair[0]..=(pair[0] + pair[1] - 1)).into())
        .collect();
    seed_ranges.sort();

    // find the ranges that will transform into lowest location in the condensed map.
    let mut seed_domains: Vec<DisjointRange> = seed_ranges
        .iter()
        .flat_map(|range| condensed_map.map_range(*range))
        .collect();

    seed_domains.sort();

    Ok(seed_domains[0].start)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn test_mapping(#[case] seed: u64, #[case] location: u64) {
        let fixture = include_str!("../test-1.txt");

        let (_, (_, listings)) = full(fixture).expect("Parse successful.");

        let condensed_map = listings
            .into_iter()
            .map(|idmap| idmap.mappings)
            .reduce(|lhs, rhs| lhs.concatenate(&rhs))
            .expect("Condense map should not fail.");

        assert_eq!(condensed_map.map(seed), location);
    }
}
