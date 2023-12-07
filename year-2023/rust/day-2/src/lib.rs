// region:    --- Modules

use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::value,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

// endregion: --- Modules

pub fn part1(input: &str) -> Result<u32> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect();

    let legal_games: Vec<Game> = games
        .into_iter()
        .filter(|game| game.is_possbile(12, 13, 14))
        .collect();

    Ok(legal_games.iter().map(|game| game.id).sum())
}

pub fn part2(input: &str) -> Result<u32> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect();

    let games_power: Vec<_> = games.into_iter().map(|game| game.power()).collect();

    Ok(games_power.iter().sum())
}

// region:    --- Types

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub type GameSet = HashMap<Color, u32>;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    sets: Vec<GameSet>,
}

// endregion: --- Types

// region:    --- Parser

fn color(input: &str) -> IResult<&str, (Color, u32)> {
    let (input, quantity) = preceded(space0, digit1)(input)?;
    let quantity = quantity.parse().unwrap();

    let (rest, color) = alt((
        value(Color::Red, tag(" red")),
        value(Color::Green, tag(" green")),
        value(Color::Blue, tag(" blue")),
    ))(input)?;

    Ok((rest, (color, quantity)))
}
fn gameset(input: &str) -> IResult<&str, GameSet> {
    let (rest, colors) = separated_list1(tag(","), color)(input)?;

    let mut gameset = GameSet::new();
    for (color, quantity) in colors {
        gameset.insert(color, quantity);
    }

    Ok((rest, gameset))
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let id = id.parse().unwrap();
    let (input, _) = tag(":")(input)?;

    let (_, sets) = separated_list1(tag("; "), gameset)(input)?;

    Ok(("", Game { id, sets }))
}

// endregion: --- Parser

// region:    --- Part 1

impl Game {
    pub fn is_possbile(&self, red_limit: u32, green_limit: u32, blue_limit: u32) -> bool {
        for set in &self.sets {
            if let Some(red) = set.get(&Color::Red) {
                if *red > red_limit {
                    return false;
                }
            }
            if let Some(green) = set.get(&Color::Green) {
                if *green > green_limit {
                    return false;
                }
            }
            if let Some(blue) = set.get(&Color::Blue) {
                if *blue > blue_limit {
                    return false;
                }
            }
        }
        true
    }
}
// endregion: --- Part 1

// region:    --- Part 2

impl Game {
    fn needed(&self, color: Color) -> u32 {
        self.sets
            .iter()
            .map(|set| match set.get(&color) {
                Some(val) => *val,
                None => 0,
            })
            .max()
            .unwrap_or(0)
    }

    pub fn power(&self) -> u32 {
        let red = self.needed(Color::Red);
        let green = self.needed(Color::Green);
        let blue = self.needed(Color::Blue);

        red * green * blue
    }
}

// endregion: --- Part 2

// region:    --- Tests

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}

    #[rstest]
    #[case(true, "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case(
        true,
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
    )]
    #[case(
        false,
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
    )]
    #[case(
        false,
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
    )]
    #[case(true, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")]
    fn test_game_is_possible(#[case] expected: bool, #[case] input: &str) {
        let game = parse_game(input).unwrap().1;
        assert_eq!(expected, game.is_possbile(12, 13, 14));
    }
}
// endregion: --- Tests
