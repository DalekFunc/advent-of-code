use day_13::part2;

fn main() {
    let result = part2(include_bytes!("../../input.txt")).expect("Part 2 failed to run");

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(400)]
    fn test_using_test_input_1(#[case] expected: u64) {
        let result = part2(include_bytes!("../../test-1.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(700)]
    fn test_using_test_input_2(#[case] expected: u64) {
        let result = part2(include_bytes!("../../test-2.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
