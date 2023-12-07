use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    combinator::eof,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub have_numbers: Vec<u32>,
}

// region:    --- Parsing
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, id) = delimited(space0, u32, tag(":"))(input)?;
    let (input, mut winning_numbers) =
        delimited(space0, separated_list1(space1, u32), tag(" | "))(input)?;
    let (_, mut have_numbers) = delimited(space0, separated_list1(space1, u32), eof)(input)?;

    winning_numbers.sort();
    have_numbers.sort();

    Ok((
        "",
        Card {
            id,
            winning_numbers,
            have_numbers,
        },
    ))
}

// endregion: --- Parsing

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_card(s)
            .map(|result| result.1)
            .map_err(|e| format!("Card Parse Error: {e:?}"))
    }
}

impl Card {
    pub fn point(&self) -> u32 {
        let mut winning = self.winning_numbers.iter().peekable();
        let have = self.have_numbers.iter();
        let base_point = 1;
        let mut point = 0;

        for num in have {
            loop {
                if winning.peek().is_none() {
                    break;
                }

                let win = *winning.peek().expect("winning is not none.");
                match num.cmp(win) {
                    std::cmp::Ordering::Less => break,
                    std::cmp::Ordering::Equal => {
                        if point == 0 {
                            point = base_point
                        } else {
                            point *= 2
                        }
                        winning.next();
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        winning.next();
                    }
                }
            }
        }

        point
    }

    pub fn matching(&self) -> u32 {
        let mut winning = self.winning_numbers.iter().peekable();
        let have = self.have_numbers.iter();
        let mut point = 0;

        for num in have {
            loop {
                if winning.peek().is_none() {
                    break;
                }

                let win = *winning.peek().expect("winning is not none.");
                match num.cmp(win) {
                    std::cmp::Ordering::Less => break,
                    std::cmp::Ordering::Equal => {
                        point += 1;
                        winning.next();
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        winning.next();
                    }
                }
            }
        }

        point
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::Card;

    #[test]
    fn test_parsing() {
        let fixture = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = fixture.parse::<Card>().expect("Should parse successfully");

        assert_eq!(1, card.id);
        assert_eq!(vec![17, 41, 48, 83, 86], card.winning_numbers);
        assert_eq!(vec![6, 9, 17, 31, 48, 53, 83, 86], card.have_numbers);
    }

    #[test]
    fn test_point() {
        let fixture = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = fixture.parse::<Card>().expect("Should parse successfully");

        assert_eq!(8, card.point());
    }
}
