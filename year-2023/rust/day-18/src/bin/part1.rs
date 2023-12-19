use day_18::part1;

fn main() {
    let result = part1(include_str!("../../input.txt")).expect("Part 1 failed to run");

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(62)]
    fn test_using_test_input_1(#[case] expected: u64) {
        let result = part1(include_str!("../../test-1.txt")).expect("Part 1 failed to run");

        assert_eq!(expected, result);
    }
}
