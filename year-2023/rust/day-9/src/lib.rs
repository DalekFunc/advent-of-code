// region:    --- Modules
use anyhow::Result;
use math::ncr;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};
mod math;
// endregion: --- Modules

pub fn part1(input: &str) -> Result<i64> {
    let list_of_numbers = parse_file(input);

    Ok(list_of_numbers
        .into_iter()
        .map(|numbers| extrapolate(&numbers))
        .sum())
}

pub fn part2(input: &str) -> Result<i64> {
    let list_of_numbers = parse_file(input);

    Ok(list_of_numbers
        .into_iter()
        .map(|numbers| extrapolate_backward(&numbers))
        .sum())
}

// region:    --- Parsing
fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64)(input)
}

fn parse_file(input: &str) -> Vec<Vec<i64>> {
    let (_, vec_of_numbers) =
        all_consuming(separated_list1(line_ending, parse_line))(input).expect("parse complete");
    vec_of_numbers
}
// endregion: --- Parsing

// region:    --- Part 1

fn all_equal<T>(mut iter: impl Iterator<Item = T>) -> bool
where
    T: Eq,
{
    let first_elem = iter.next();
    let Some(first_elem) = first_elem else {
        return true;
    };

    iter.all(|elem| elem == first_elem)
}

fn find_seeds(numbers: &[i64]) -> Vec<i64> {
    assert!(numbers.len() > 1);

    match numbers.len() {
        2 => {
            if numbers[0] == numbers[1] {
                vec![numbers[0]]
            } else {
                vec![numbers[0], numbers[1] - numbers[0]]
            }
        }
        _ => {
            let diffs: Vec<_> = numbers
                .windows(2)
                .map(|pairs| pairs[1] - pairs[0])
                .collect();

            if all_equal(diffs.iter()) {
                vec![numbers[0], diffs[0]]
            } else {
                let mut result = vec![numbers[0]];
                result.append(&mut find_seeds(&diffs));
                result
            }
        }
    }
}

fn extrapolate(numbers: &[i64]) -> i64 {
    let seeds = find_seeds(numbers);

    seeds
        .into_iter()
        .enumerate()
        .map(|(r, seed)| ncr(numbers.len() as u64, r as u64) as i64 * seed)
        .sum()
}

// endregion: --- Part 1

// region:    --- Part 2

fn extrapolate_backward(numbers: &[i64]) -> i64 {
    let mut seeds = find_seeds(numbers);
    seeds.reverse();

    seeds.into_iter().reduce(|acc, rhs| rhs - acc).unwrap()
}

// endregion: --- Part 2

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        let (_, numbers) = parse_line("10 13 16 21 30 45").expect("parse successful");
        println!("{:?}", find_seeds(&numbers));
        let (_, numbers) = parse_line("22 31 52 90 144 206 260 281 234 73 -260 -836 -1740 -3072 -4948 -7501 -10882 -15261 -20828 -27794 -36392").expect("parse successful");
        println!("{:?}", find_seeds(&numbers));
        let (_, numbers) = parse_line("12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498 8487217 17211396").expect("parse successful");
        let n_len = numbers.len();
        println!("{n_len}");
        let seeds = find_seeds(&numbers);
        println!("{:?}", seeds.len());
    }

    #[rstest]
    #[case(true, vec![])]
    #[case(true, vec![0])]
    #[case(true, vec![0, 0])]
    #[case(true, vec![0, 0, 0])]
    #[case(false, vec![0, 1])]
    #[case(false, vec![0, 1, 1])]
    #[case(false, vec![0, 0, 1])]
    #[case(false, vec![0, 1, 0])]
    #[case(false, vec![0, 3])]
    #[case(false, vec![1, 2, 1])]
    #[case(false, vec![10, 3, 0, 2])]
    fn test_all_equal(#[case] expected: bool, #[case] input: Vec<i64>) {
        assert_eq!(expected, all_equal(input.iter()));
    }

    #[rstest]
    #[case(vec![2, 1], "2 3 4")]
    #[case(vec![3, 0, 2], "3 3 5")]
    #[case(vec![0, 3], "0 3 6 9 12 15")]
    #[case(vec![1, 2, 1], "1 3 6 10 15 21")]
    #[case(vec![10, 3, 0, 2], "10 13 16 21 30 45")]
    #[case(vec![12, 6, 15, 15, 13, 17, 19, 14, 3, 15, 23, 23, 20, 17, 4, 2, 3, -4], "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697")]
    #[case(vec![12, 6, 15, 15, 13, 17, 19, 14, 3, 15, 23, 23, 20, 17, 4, 2, 3, -4], "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498")]
    #[case(vec![12, 6, 15, 15, 13, 17, 19, 14, 3, 15, 23, 23, 20, 17, 4, 2, 3, -4], "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498 8487217")]
    #[case(vec![12, 6, 15, 15, 13, 17, 19, 14, 3, 15, 23, 23, 20, 17, 4, 2, 3, -4], "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498 8487217 17211396")]
    fn test_find_seed_and_diff(#[case] expected: Vec<i64>, #[case] input: &str) {
        let (_, numbers) = parse_line(input).expect("parse successful");
        assert_eq!(expected, find_seeds(&numbers));
    }

    #[rstest]
    #[case(
        156,
        "9 16 23 30 37 44 51 58 65 72 79 86 93 100 107 114 121 128 135 142 149"
    )]
    #[case(-46878, "22 31 52 90 144 206 260 281 234 73 -260 -836 -1740 -3072 -4948 -7501 -10882 -15261 -20828 -27794 -36392
    ")]
    #[case(153, "15 38 84")]
    #[case(-27, "-6 -7 -8 -9 -10 -11 -12 -13 -14 -15 -16 -17 -18 -19 -20 -21 -22 -23 -24 -25 -26
    ")]
    #[case(4128498, "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697")]
    #[case(8487217, "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498")]
    #[case(17211396, "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498 8487217")]
    #[case(34298953, "12 18 39 90 199 424 889 1853 3829 7788 15539 30516 59516 116587 231569 468274 961829 1992697 4128498 8487217 17211396")]
    #[case(18, "0 3 6 9 12 15")]
    #[case(28, "1 3 6 10 15 21")]
    #[case(68, "10 13 16 21 30 45")]
    fn test_extrapolation(#[case] expected: i64, #[case] input: &str) {
        let (_, numbers) = parse_line(input).expect("parse successful");
        assert_eq!(expected, extrapolate(&numbers));
    }
}
