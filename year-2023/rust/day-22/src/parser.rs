use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use crate::{brick::Brick, coordinates::Coord3D};

pub fn coord3d(input: &str) -> IResult<&str, Coord3D> {
    separated_list1(tag(","), complete::u32)
        .map(|vec| Coord3D {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        })
        .parse(input)
}
pub fn block(input: &str) -> IResult<&str, Brick> {
    separated_pair(coord3d, tag("~"), coord3d)
        .map(|(p1, p2)| Brick::new(p1, p2))
        .parse(input)
}

pub fn parse_file(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(newline, block)(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        let fixture = "2,3,283~4,3,283";

        let block = block(fixture).expect("parse ok");

        println!("{:?}", block);
    }
}
