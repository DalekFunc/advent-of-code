use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_direction(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (rest, (from, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)?;
    Ok((rest, (from, left, right)))
}

pub fn parse_document(
    input: &str,
) -> IResult<&str, (&str, HashMap<&str, &str>, HashMap<&str, &str>)> {
    let (rest, (instructions, directions)) = separated_pair(
        alpha1,
        tag("\n\n"),
        separated_list1(line_ending, parse_direction),
    )(input)?;

    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    directions.into_iter().for_each(|(from, left, right)| {
        left_map.insert(from, left);
        right_map.insert(from, right);
    });

    Ok((rest, (instructions, left_map, right_map)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn quick_test() {}

    #[test]
    fn test_parsing() {
        let result = parse_document(include_str!("../test-1.txt"));
        assert!(result.is_ok());
        println!("{result:?}");
    }
}
