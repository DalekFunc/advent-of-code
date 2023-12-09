pub fn factorial(n: u64) -> u64 {
    assert!(n >= 0);
    if n == 0 {
        return 1;
    }
    (1..=n).product()
}

pub fn ncr(n: u64, r: u64) -> u64 {
    assert!(n > 0);

    (n - r + 1..=n).product::<u64>() / factorial(r)
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(6, 3)]
    #[case(24, 4)]
    fn test_factorial(#[case] expected: u64, #[case] input: u64) {
        assert_eq!(expected, factorial(input));
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(1, 2, 0)]
    #[case(1, 128, 0)]
    #[case(1, 2, 2)]
    #[case(10, 5, 2)]
    #[case(15, 6, 4)]
    fn test_ncr(#[case] expected: u64, #[case] n: u64, #[case] r: u64) {
        assert_eq!(expected, ncr(n, r));
    }
}
