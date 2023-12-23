use day_6::part2;

fn main() {
    let result = part2(include_str!("../../input.txt")).expect("Part 2 failed to run");

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(71503)]
    fn test_using_test_input_1(#[case] expected: u64) {
        let result = part2(include_str!("../../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
