use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{peek, value},
    multi::many0,
    IResult,
};

pub fn part1(input: &str) -> Result<u32> {
    Ok(input.lines().flat_map(|line| extract(line)).sum())
}

pub fn part2(input: &str) -> Result<u32> {
    Ok(input.lines().flat_map(|line| extract2(line)).sum())
}

// region:    --- Part 1

fn extract(input: &str) -> Option<u32> {
    let mut digits = input.chars().filter(|c| c.is_numeric()).peekable();

    let Some(l) = digits.peek() else { return None };
    let l = l.clone();
    let Some(r) = digits.last() else { return None };

    let num_str = format!("{}{}", l, r);
    num_str.parse().ok()
}

// endregion: --- Part 1

// region:    --- Part 2

// if matches, the parse consume ON, but not E, as it can be used later for Eight.
fn one(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("one"))(input)?;
    let (rest, _) = tag("on")(input)?;
    Ok((rest, Some("1")))
}

fn two(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("two"))(input)?;
    let (rest, _) = tag("tw")(input)?;
    Ok((rest, Some("2")))
}

fn three(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("three"))(input)?;
    let (rest, _) = tag("thr")(input)?;
    Ok((rest, Some("3")))
}

fn five(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("five"))(input)?;
    let (rest, _) = tag("fiv")(input)?;
    Ok((rest, Some("5")))
}

fn seven(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("seven"))(input)?;
    let (rest, _) = tag("seve")(input)?;
    Ok((rest, Some("7")))
}

fn eight(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("eight"))(input)?;
    let (rest, _) = tag("eigh")(input)?;
    Ok((rest, Some("8")))
}

fn nine(input: &str) -> IResult<&str, Option<&str>> {
    let (input, _) = peek(tag("nine"))(input)?;
    let (rest, _) = tag("nin")(input)?;
    Ok((rest, Some("9")))
}

fn get_digit_or_junk(input: &str) -> IResult<&str, Option<&str>> {
    alt((
        one,
        two,
        three,
        value(Some("4"), tag("four")),
        five,
        value(Some("6"), tag("six")),
        seven,
        eight,
        nine,
        value(Some("1"), tag("1")),
        value(Some("2"), tag("2")),
        value(Some("3"), tag("3")),
        value(Some("4"), tag("4")),
        value(Some("5"), tag("5")),
        value(Some("6"), tag("6")),
        value(Some("7"), tag("7")),
        value(Some("8"), tag("8")),
        value(Some("9"), tag("9")),
        value(Some("0"), tag("0")),
        value(None, anychar),
    ))(input)
}

fn extract2(input: &str) -> Option<u32> {
    let (_rest, tokens) = many0(get_digit_or_junk)(input).unwrap();
    let mut digits = tokens
        .into_iter()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .peekable();

    let Some(l) = digits.peek() else { return None };
    let l = *l;
    let Some(r) = digits.last() else { return None };

    let num_str = format!("{}{}", l, r);
    num_str.parse().ok()
}

// endregion: --- Part 2

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_parse_multiple() {
        let fixture = "a1b2c3d4e5f";
        assert_eq!(15, extract(fixture).unwrap());
    }

    #[test]
    fn test_parse_single() {
        let fixture = "treb7uchet";
        assert_eq!(77, extract(fixture).unwrap());
    }

    #[rstest]
    #[case(29, "two1nine")]
    #[case(83, "eightwothree")]
    #[case(13, "abcone2threexyz")]
    #[case(24, "xtwone3four")]
    #[case(42, "4nineeightseven2")]
    #[case(14, "zoneight234")]
    #[case(76, "7pqrstsixteen")]
    #[case(88, "eight")]
    #[case(11, "1")]
    #[case(11, "111111")]
    #[case(21, "twone")]
    fn test_extract2(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, extract2(input).unwrap());
    }
}
