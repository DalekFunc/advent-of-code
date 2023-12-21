use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of(".#S"))(input)
}

pub fn parse_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(newline, parse_line)(input)
}