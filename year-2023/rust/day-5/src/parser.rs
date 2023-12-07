// region:    --- Modules
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::{all_consuming, eof},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use crate::range_map::RangeMap;
// endregion: --- Modules

#[derive(Debug)]
pub struct IDMap {
    pub source: String,
    pub target: String,
    pub mappings: RangeMap,
}

impl IDMap {
    pub fn transfer(&self, source: u64) -> u64 {
        self.mappings.map(source)
    }
}

// region:    --- Parsing

fn id_map(input: &str) -> IResult<&str, IDMap> {
    let (input, (source, target)) =
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"))(input)?;
    let (rest, mut mappings_raw) = terminated(
        separated_list1(line_ending, separated_list1(tag(" "), complete::u64)),
        alt((line_ending, eof)),
    )(input)?;

    let source = source.to_owned();
    let target = target.to_owned();
    mappings_raw.sort_by(|lhs, rhs| lhs[1].cmp(&rhs[1]));

    // build RangeMap here
    let mut mappings = RangeMap::build();
    for mapping in mappings_raw {
        mappings = mappings.insert(
            (mapping[1]..=mapping[1] + mapping[2] - 1).into(),
            (mapping[0]..=mapping[0] + mapping[2] - 1).into(),
        );
    }
    let mappings = mappings.fill_gaps();

    Ok((
        rest,
        IDMap {
            source,
            target,
            mappings,
        },
    ))
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("seeds: "),
        separated_list1(tag(" "), complete::u64),
        tag("\n\n"),
    )(input)
}

fn listings(input: &str) -> IResult<&str, Vec<IDMap>> {
    separated_list1(line_ending, id_map)(input)
}

pub fn full(input: &str) -> IResult<&str, (Vec<u64>, Vec<IDMap>)> {
    let (input, seeds) = seeds(input)?;
    let (_, listings) = all_consuming(listings)(input)?;

    assert!(listings.len() == 7);

    Ok(("", (seeds, listings)))
}

// endregion: --- Parsing

#[cfg(test)]
mod tests {
    use crate::parser::full;

    use super::id_map;

    #[test]
    fn test_id_map_parsing() {
        let fixture = "seed-to-soil map:
50 98 2
52 50 48
";
        let map = id_map(fixture);
        println!("{:?}", map);
    }

    #[test]
    fn test_full_parsing() {
        let fixture = include_str!("../test-1.txt");
        let all = full(fixture);

        println!("{:?}", all);
    }
}
