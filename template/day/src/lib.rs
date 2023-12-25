use anyhow::{Result, anyhow};

pub fn part1(input: &str) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
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
    #[case(0)]
    fn part_1_using_test_input_1(#[case] expected: u64) {
        let result = part1(include_str!("../test-1.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(0)]
    fn part_2_using_test_input_1(#[case] expected: u64) {
        let result = part2(include_str!("../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}