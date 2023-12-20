use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::simulation::{module::Module, FlipFlopState};

fn receivers(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn module(input: &str) -> IResult<&str, Module> {
    alt((
        preceded(tag("broadcaster -> "), receivers).map(|receivers| Module::Broadcaster {
            name: "broadcaster",
            receivers,
        }),
        preceded(tag("%"), separated_pair(alpha1, tag(" -> "), receivers)).map(
            |(name, receivers)| Module::FlipFlop {
                name,
                state: FlipFlopState::OFF,
                receivers,
            },
        ),
        preceded(tag("&"), separated_pair(alpha1, tag(" -> "), receivers)).map(
            |(name, receivers)| Module::Conjunction {
                name,
                record: HashMap::new(),
                receivers,
            },
        ),
    ))(input)
}

pub fn parse_file(input: &str) -> IResult<&str, Vec<Module>> {
    separated_list1(newline, module)(input)
}
