use day_10::part2;

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
    #[case(4)]
    fn test_using_test_input_3(#[case] expected: u64) {
        let result = part2(include_bytes!("../../test-3.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(8)]
    fn test_using_test_input_4(#[case] expected: u64) {
        let result = part2(include_bytes!("../../test-4.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(10)]
    fn test_using_test_input_5(#[case] expected: u64) {
        let result = part2(include_bytes!("../../test-5.txt")).expect("Part 2 failed to run");

        assert_eq!(expected, result);
    }
}
