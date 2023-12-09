// region:    --- Modules
use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::HashMap;
// endregion: --- Modules

pub fn part1(input: &str) -> Result<i32> {
    let (_, rel_map) = parse_relationships(input).expect("parse success");
    let people: Vec<&str> = rel_map.keys().cloned().collect();

    let first_person = *people.first().expect("people not empty");
    let sitting_cycles = people[1..]
        .iter()
        .cloned()
        .permutations(people.len() - 1)
        .map(|mut vec| {
            vec.push(first_person);
            vec
        })
        .collect::<Vec<_>>();

    // dbg!(&sitting_cycles);

    Ok(sitting_cycles
        .iter()
        .map(|cycle| {
            let mut sum = cycle
                .windows(2)
                .map(|pair| rel_map[pair[0]][pair[1]] + rel_map[pair[1]][pair[0]])
                .sum();
            sum += rel_map[cycle[0]][cycle[cycle.len() - 1]]
                + rel_map[cycle[cycle.len() - 1]][cycle[0]];
            dbg!(&sum);

            sum
        })
        .max()
        .expect("cycle not empty"))
}

pub fn part2(input: &str) -> Result<i32> {
    let (_, rel_map) = parse_relationships(input).expect("parse success");
    let people: Vec<&str> = rel_map.keys().cloned().collect();

    let sitting_cycles = people
        .iter()
        .cloned()
        .permutations(people.len() - 1)
        .collect::<Vec<_>>();

    // dbg!(&sitting_cycles);

    Ok(sitting_cycles
        .iter()
        .map(|cycle| {
            dbg!(&cycle);
            let sum = cycle
                .windows(2)
                .map(|pair| rel_map[pair[0]][pair[1]] + rel_map[pair[1]][pair[0]])
                .sum();
            dbg!(&sum);

            sum
        })
        .max()
        .expect("cycle not empty"))
}
// region:    --- Parsing
type RelationshipMap<'a> = HashMap<&'a str, HashMap<&'a str, i32>>;
fn parse_relationship(input: &str) -> IResult<&str, (&str, i32, &str)> {
    let (input, person) = alpha1(input)?;
    let (input, happiness) = preceded(
        tag(" would "),
        alt((
            (preceded(tag("gain "), complete::i32)),
            map(preceded(tag("lose "), complete::i32), |value: i32| -value),
        )),
    )(input)?;
    let (rest, neighbour) = delimited(
        tag(" happiness units by sitting next to "),
        alpha1,
        tag("."),
    )(input)?;

    Ok((rest, (person, happiness, neighbour)))
}

fn parse_relationships(input: &str) -> IResult<&str, RelationshipMap> {
    let mut r_map = RelationshipMap::new();
    let (rest, relationships) = separated_list1(line_ending, parse_relationship)(input)?;
    relationships.into_iter().for_each(|(p, h, n)| {
        r_map
            .entry(p)
            .and_modify(|book| {
                book.insert(n, h);
            })
            .or_insert({
                let mut book: HashMap<&str, i32> = HashMap::new();
                book.insert(n, h);
                book
            });
    });

    Ok((rest, r_map))
}
// endregion: --- Parsing

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[test]
    fn test_parsing() {
        let result = parse_relationships(include_str!("../test-1.txt"))
            .unwrap()
            .1;
        println!("{:?}", result);
    }
}
