use anyhow::{Result, anyhow};
use nom::bytes::complete::take_until;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use nom::bytes::complete::is_not;

pub fn part1(input: &[u8]) -> Result<u64> {
    let (_, strings) = parse_file(input).expect("parse ok");

    println!("{:?}" , strings);

    fn process(value: &mut u32, character: u32) {
        *value += character;
        *value *= 17;
        *value %= 256;
    }

    let sum = strings.iter().fold(0, |acc, string|
        acc + {
            let mut value = 0;
            string.iter().for_each(|&character| process(&mut value, character.into()));
            dbg!(value)
        });


    Ok(sum.into())
}

pub fn part2(input: &[u8]) -> Result<u64> {
    Err(anyhow!("Not Implemented."))
}

fn parse_file(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list1(
        tag(b","), is_not(",")
    )(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}