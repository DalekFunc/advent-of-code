use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    combinator::eof,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

pub fn part1(input: &str) -> Result<u64> {
    let (_, (time, dist)) = parse_time_distance(input).unwrap();

    Ok(time
        .iter()
        .zip(dist.iter())
        .map(|(time, record)| numbers_of_ways_to_win(*time, *record))
        .product::<u64>())
}

pub fn part2(input: &str) -> Result<u64> {
    let (_, (times, dists)) = parse_time_distance(input).unwrap();

    let time = times
        .into_iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let record = dists
        .into_iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    dbg!(&time);
    dbg!(&record);

    Ok(numbers_of_ways_to_win(time, record))
}

// region:    --- Parsing

fn parse_time_distance(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, times) = delimited(
        tuple((tag("Time:"), space0)),
        separated_list1(space1, complete::u64),
        line_ending,
    )(input)?;
    let (rest, distances) = delimited(
        tuple((tag("Distance:"), space0)),
        separated_list1(space1, complete::u64),
        eof,
    )(input)?;

    Ok((rest, (times, distances)))
}

// endregion: --- Parsing

// region:    --- Part 1

fn distance_travelled(acceleration: u64, hold: u64, time: u64) -> u64 {
    (time - hold) * (acceleration * hold)
}

fn numbers_of_ways_to_win(time: u64, record: u64) -> u64 {
    (1..time)
        .filter(|hold| distance_travelled(1, *hold, time) > record)
        .count() as u64
}

// endregion: --- Part 1

// region:    --- Part 2

// endregion: --- Part 2

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        let (_, (time, dist)) =
            parse_time_distance(include_str!("../test-1.txt")).expect("Parsing should not fail.");

        println!("{:?}", time);
        println!("{:?}", dist);
    }

    #[rstest]
    #[case(1, 0, 7, 0)]
    #[case(1, 1, 7, 6)]
    #[case(1, 2, 7, 10)]
    #[case(1, 3, 7, 12)]
    #[case(1, 4, 7, 12)]
    #[case(1, 5, 7, 10)]
    #[case(1, 6, 7, 6)]
    #[case(1, 7, 7, 0)]
    fn test_distance_travelled(
        #[case] acceleration: u64,
        #[case] hold: u64,
        #[case] time: u64,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, distance_travelled(acceleration, hold, time))
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn test_numbers_of_ways_to_win(#[case] record: u64, #[case] time: u64, #[case] expected: u64) {
        assert_eq!(expected, numbers_of_ways_to_win(record, time))
    }
}
