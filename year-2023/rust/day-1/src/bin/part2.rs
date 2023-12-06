use day_1::part2;

fn main() {
    let result = part2(include_str!("../input.txt")).expect("Part 2 failed to run");

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    fn test_using_test_input_2() {
        let result = part2(include_str!("../test-2.txt")).expect("Part 2 failed to run");

        let expected = 281;

        assert_eq!(expected, result);
    }
}
