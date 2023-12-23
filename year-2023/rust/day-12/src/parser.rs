use nom::{
    bytes::complete::{is_a, tag, take_till},
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use crate::token::Token;

// part 1
pub fn parse_line(input: &str) -> IResult<&str, (Vec<u8>, Vec<u32>)> {
    let (rest, (space, arrangement)) = separated_pair(
        take_till(|c| c == ' '),
        tag(" "),
        separated_list1(tag(","), complete::u32),
    )(input)?;

    Ok((rest, (space.as_bytes().to_owned(), arrangement)))
}

// part 2
pub fn tokens(input: &str) -> IResult<&str, Vec<Token>> {
    is_a(".?#")
        .map(|string: &str| {
            string
                .chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Token::Empty,
                    '#' => Token::Block,
                    '?' => Token::Uncertain,
                    _ => unreachable!("invalid character"),
                })
                .collect::<Vec<Token>>()
        })
        .parse(input)
}

pub fn parse_line2(input: &str) -> IResult<&str, (Vec<Token>, Vec<u8>)> {
    let (rest, (space, arrangement)) =
        separated_pair(tokens, tag(" "), separated_list1(tag(","), complete::u8))(input)?;

    Ok((rest, (space, arrangement)))
}
