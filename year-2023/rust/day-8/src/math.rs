pub fn factorize(mut number: u64) -> Vec<u64> {
    let mut factors = vec![];

    loop {
        if number == 1 {
            break;
        }
        for divisor in 2.. {
            if number % divisor == 0 {
                factors.push(divisor);
                number /= divisor;
                break;
            }
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(vec![2, 2, 2, 3], 24)]
    #[case(vec![37], 37)]
    fn test_factorize(#[case] expected: Vec<u64>, #[case] input: u64) {
        assert_eq!(expected, factorize(input));
    }
}
