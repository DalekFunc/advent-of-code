use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::BTreeMap;

pub fn part1(input: &[u8]) -> Result<u64> {
    let (_, strings) = parse_file(input).expect("parse ok");

    let sum = strings.iter().fold(0, |acc, string| acc + hash(string));

    Ok(sum.into())
}

pub fn part2(input: &[u8]) -> Result<u64> {
    let (_, strings) = parse_file(input).expect("parse ok");

    let mut boxes = BTreeMap::new();

    strings.into_iter().for_each(|string| {
        if string.contains(&b'=') {
            let pair: Vec<_> = string.split(|&character| character == b'=').collect();
            let label = pair[0];
            let new_focal_len = std::str::from_utf8(pair[1]).unwrap().parse().unwrap();
            let box_id = hash(label);

            boxes
                .entry(box_id)
                .and_modify(|lenses: &mut Vec<(&[u8], u64)>| {
                    if let Some(id) = lenses
                        .iter()
                        .positions(|(l, _)| l == &label)
                        .at_most_one()
                        .expect("at most one lens with label")
                    {
                        lenses[id].1 = new_focal_len;
                    } else {
                        lenses.push((label, new_focal_len));
                    }
                })
                .or_insert(vec![(label, new_focal_len)]);
        } else {
            // contains '-'
            let pair: Vec<_> = string.split(|&character| character == b'-').collect();
            let label = pair[0];
            let box_id = hash(label);

            boxes.entry(box_id).and_modify(|lenses| {
                if let Some(id) = lenses
                    .iter()
                    .positions(|(l, _)| l == &label)
                    .at_most_one()
                    .expect("at most one lens with label")
                {
                    lenses.remove(id);
                }
            });
        }
    });

    Ok(boxes.iter().fold(0, |acc, (id, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .map(|(slot, (_, focal_len))| (id + 1) as u64 * (slot + 1) as u64 * *focal_len)
            .sum::<u64>()
    }))
}

fn parse_file(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list1(tag(b","), is_not(","))(input)
}

fn hash(label: &[u8]) -> u32 {
    fn process(value: &mut u32, character: u32) {
        *value += character;
        *value *= 17;
        *value %= 256;
    }

    let mut value = 0;
    label
        .iter()
        .for_each(|&character| process(&mut value, character.into()));
    value
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}
