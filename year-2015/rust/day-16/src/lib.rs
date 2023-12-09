use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn part1(input: &str) -> Result<u64> {
    let (_, aunt_props) = parse_aunts(input).expect("parse ok");

    let mut details = HashMap::<&str, u32>::new();
    details.insert("children", 3);
    details.insert("cats", 7);
    details.insert("samoyeds", 2);
    details.insert("pomeranians", 3);
    details.insert("akitas", 0);
    details.insert("vizslas", 0);
    details.insert("goldfish", 5);
    details.insert("trees", 3);
    details.insert("cars", 2);
    details.insert("perfumes", 1);

    let aunts: Vec<_> = aunt_props
        .iter()
        .filter(|aunt| aunt.1.iter().all(|prop| details[prop.0] == prop.1))
        .collect();

    Ok(aunts[0].0 as u64)
}

pub fn part2(input: &str) -> Result<u64> {
    let (_, aunt_props) = parse_aunts(input).expect("parse ok");

    let mut details = HashMap::<&str, u32>::new();
    details.insert("children", 3);
    details.insert("cats", 7);
    details.insert("samoyeds", 2);
    details.insert("pomeranians", 3);
    details.insert("akitas", 0);
    details.insert("vizslas", 0);
    details.insert("goldfish", 5);
    details.insert("trees", 3);
    details.insert("cars", 2);
    details.insert("perfumes", 1);

    let aunts: Vec<_> = aunt_props
        .iter()
        .filter(|aunt| {
            aunt.1.iter().all(|prop| match prop {
                prop if prop.0 == "cats" || prop.0 == "trees" => details[prop.0] < prop.1,
                prop if prop.0 == "pomeranians" || prop.0 == "goldfish" => details[prop.0] > prop.1,
                prop => details[prop.0] == prop.1,
            })
        })
        .collect();

    Ok(aunts[0].0 as u64)
}

// region:    --- Parser

type AuntProp<'a> = (u32, Vec<(&'a str, u32)>);

fn parse_aunt(input: &str) -> IResult<&str, AuntProp> {
    let (input, id) = delimited(tag("Sue "), complete::u32, tag(": "))(input)?;
    let (rest, props) =
        separated_list1(tag(", "), separated_pair(alpha1, tag(": "), complete::u32))(input)?;

    Ok((rest, (id, props)))
}

fn parse_aunts(input: &str) -> IResult<&str, Vec<AuntProp>> {
    separated_list1(line_ending, parse_aunt)(input)
}

// endregion: --- Parser

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[test]
    fn test_parsing() {
        let result = parse_aunts(include_str!("../test-1.txt")).unwrap().1;
        println!("{result:?}");
    }
}
