use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn angle(&self) -> f64 {
        self.x / self.y
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct HailStone {
    pub pos: Vector2D,
    pub spd: Vector2D,
}

// const OFFSET: i64 = 200000000000000;
const OFFSET: i64 = 0;
// const OFFSET: i64 = 7;

fn coord_2d(input: &str) -> IResult<&str, Vector2D> {
    let (rest, nums) = separated_list1(tag(","), preceded(space0, complete::i64))(input)?;

    let coord = Vector2D {
        x: (nums[0] - OFFSET) as f64, // lossy, should be ok for our input range
        y: (nums[1] - OFFSET) as f64,
    };

    Ok((rest, coord))
}

fn hailstone(input: &str) -> IResult<&str, HailStone> {
    separated_pair(coord_2d, tag(" @ "), coord_2d)
        .map(|(position, speed)| HailStone {
            pos: position,
            spd: speed,
        })
        .parse(input)
}

pub fn parse_file(input: &str) -> IResult<&str, Vec<HailStone>> {
    separated_list1(line_ending, hailstone)(input)
}
