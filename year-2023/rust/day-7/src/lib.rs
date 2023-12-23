// region:    --- Modules

use anyhow::Result;
use nom::{
    bytes::complete::take_while_m_n,
    character::complete::{self, space1},
    combinator::all_consuming,
    sequence::separated_pair,
    IResult,
};

// endregion: --- Modules

pub fn part1(input: &str) -> Result<u64> {
    let mut hands_and_bids: Vec<(Hand<Rank>, u64)> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    hands_and_bids.sort_by(|(lhs_hand, _), (rhs_hand, _)| lhs_hand.cmp(rhs_hand));

    Ok(hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u64 * *bid)
        .sum())
}

pub fn part2(input: &str) -> Result<u64> {
    let mut hands_and_bids: Vec<(Hand<Rank2>, u64)> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    hands_and_bids.sort_by(|(lhs_hand, _), (rhs_hand, _)| lhs_hand.cmp(rhs_hand));

    Ok(hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u64 * *bid)
        .sum())
}

// region:    --- Types

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Rank {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            c if "23456789".contains(c) => Self::Number(c.to_digit(10).unwrap() as u8),
            _ => panic!("Unexpected character"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank2 {
    Jack,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl From<char> for Rank2 {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            c if "23456789".contains(c) => Self::Number(c.to_digit(10).unwrap() as u8),
            _ => panic!("Unexpected character"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Hand<T>([T; 5]);

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Number(value) if *value == 10 => "T".into(),
                Rank::Number(value) => value.to_string(),
                Rank::Jack => "J".into(),
                Rank::Queen => "Q".into(),
                Rank::King => "K".into(),
                Rank::Ace => "A".into(),
            }
        )
    }
}

impl std::fmt::Debug for Rank2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank2::Number(value) if *value == 10 => "T".into(),
                Rank2::Number(value) => value.to_string(),
                Rank2::Jack => "J".into(),
                Rank2::Queen => "Q".into(),
                Rank2::King => "K".into(),
                Rank2::Ace => "A".into(),
            }
        )
    }
}

impl std::fmt::Debug for Hand<Rank> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{:?}, {:?}, {:?}, {:?}, {:?}]",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4],
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub trait CompareMethod {
    fn compare(lhs: &Self, rhs: &Self) -> bool;
}

// endregion: --- Types

// region:    --- Parser

pub fn parse_hand<T: From<char> + std::fmt::Debug>(input: &str) -> IResult<&str, Hand<T>> {
    take_while_m_n(5, 5, |c| "AKQJT23456789".contains(c))(input).map(|(input, ranks_str)| {
        (input, {
            let hand = Hand(
                ranks_str
                    .chars()
                    .map(T::from)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap(),
            );
            hand
        })
    })
}

pub fn parse_line<T: From<char> + std::fmt::Debug>(input: &str) -> IResult<&str, (Hand<T>, u64)> {
    all_consuming(separated_pair(parse_hand, space1, complete::u64))(input)
}

// endregion: --- Parser

// region:    --- Part 1

impl CompareMethod for Rank {
    fn compare(lhs: &Self, rhs: &Self) -> bool {
        lhs == rhs
    }
}

impl Hand<Rank> {
    // assume hand is sorted
    fn r#type(&self) -> HandType {
        let sorted_hand = {
            let mut hand = self.0.clone();
            hand.sort();
            hand
        };

        fn all_equal<T: CompareMethod>(slice: &[T]) -> bool
        where
            T: Eq,
        {
            slice
                .windows(2)
                .all(|pair| CompareMethod::compare(&pair[0], &pair[1]))
        }

        match sorted_hand {
            hand if all_equal(&hand) => HandType::FiveOfAKind,
            hand if all_equal(&hand[0..4]) || all_equal(&hand[1..5]) => HandType::FourOfAKind,
            hand if all_equal(&hand[0..3]) && hand[3] == hand[4]
                || all_equal(&hand[1..4]) && hand[0] == hand[4]
                || all_equal(&hand[2..5]) && hand[0] == hand[1] =>
            {
                HandType::FullHouse
            }
            hand if all_equal(&hand[0..3]) || all_equal(&hand[1..4]) || all_equal(&hand[2..5]) => {
                HandType::ThreeOfAKind
            }
            // AABBC AABCC, ABBCC
            hand if hand[0] == hand[1] && hand[2] == hand[3]
                || hand[0] == hand[1] && hand[3] == hand[4]
                || hand[1] == hand[2] && hand[3] == hand[4] =>
            {
                HandType::TwoPair
            }
            hand if hand.windows(2).any(|pair| pair[0] == pair[1]) => HandType::OnePair,

            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand<Rank> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Hand<Rank> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else {
            let my_type = self.r#type();
            let your_type = other.r#type();
            if my_type == your_type {
                self.0.cmp(&other.0)
            } else {
                my_type.cmp(&your_type)
            }
        }
    }
}

// endregion: --- Part 1

// region:    --- Part 2

impl CompareMethod for Rank2 {
    fn compare(lhs: &Self, rhs: &Self) -> bool {
        if *lhs == Self::Jack || *rhs == Self::Jack {
            true
        } else {
            lhs == rhs
        }
    }
}

// FIXME: they are the same impl for Rank but we are limited by rust here.
impl PartialOrd for Hand<Rank2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Hand<Rank2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else {
            let my_type = self.r#type();
            let your_type = other.r#type();
            if my_type == your_type {
                self.0.cmp(&other.0)
            } else {
                my_type.cmp(&your_type)
            }
        }
    }
}

impl Hand<Rank2> {
    // assume hand is sorted
    fn r#type(&self) -> HandType {
        let sorted_hand = {
            let mut hand = self.0.clone();
            hand.sort();
            hand
        };

        fn possible_type(hand: &[Rank2; 5]) -> HandType {
            fn all_equal(slice: &[Rank2]) -> bool {
                let card = *slice.iter().next().unwrap();
                if card == Rank2::Jack {
                    return false;
                }
                slice.iter().all(|rhs| CompareMethod::compare(&card, rhs))
            }

            match hand {
                hand if all_equal(hand) => HandType::FiveOfAKind,
                hand if all_equal(&hand[0..4]) || all_equal(&hand[1..5]) => HandType::FourOfAKind,
                hand if all_equal(&hand[0..3]) && hand[3] == hand[4]
                    || all_equal(&hand[1..4]) && hand[0] == hand[4]
                    || all_equal(&hand[2..5]) && hand[0] == hand[1] =>
                {
                    HandType::FullHouse
                }
                hand if all_equal(&hand[0..3])
                    || all_equal(&hand[1..4])
                    || all_equal(&hand[2..5]) =>
                {
                    HandType::ThreeOfAKind
                }
                // AABBC AABCC, ABBCC
                hand if hand[0] == hand[1] && hand[2] == hand[3]
                    || hand[0] == hand[1] && hand[3] == hand[4]
                    || hand[1] == hand[2] && hand[3] == hand[4] =>
                {
                    HandType::TwoPair
                }
                hand if hand
                    .windows(2)
                    .any(|pair| CompareMethod::compare(&pair[0], &pair[1])) =>
                {
                    HandType::OnePair
                }

                _ => HandType::HighCard,
            }
        }

        let number_of_jokers = sorted_hand
            .iter()
            .filter(|card| **card == Rank2::Jack)
            .count();

        match number_of_jokers {
            4 | 5 => HandType::FiveOfAKind,
            3 => {
                // AJJJB
                let h = sorted_hand;
                possible_type(&[h[3], h[0], h[1], h[2], h[4]])
            }
            2 => {
                // AJJBC
                // ABJJC
                // AJBJC
                let h = sorted_hand;
                [
                    possible_type(&[h[2], h[0], h[1], h[3], h[4]]),
                    possible_type(&[h[2], h[3], h[0], h[1], h[4]]),
                    possible_type(&[h[2], h[0], h[3], h[1], h[4]]),
                ]
                .into_iter()
                .max()
                .unwrap()
            }
            1 => {
                let h = sorted_hand;
                [
                    possible_type(&[h[0], h[1], h[2], h[3], h[4]]),
                    possible_type(&[h[1], h[0], h[2], h[3], h[4]]),
                    possible_type(&[h[1], h[2], h[0], h[3], h[4]]),
                    possible_type(&[h[1], h[2], h[3], h[0], h[4]]),
                    possible_type(&[h[1], h[2], h[3], h[4], h[0]]),
                ]
                .into_iter()
                .max()
                .unwrap()
            }
            0 => possible_type(&sorted_hand),
            _ => panic!("impossible number of jokers"),
        }
    }
}

// endregion: --- Part 2

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {
        println!("{:?}", Rank::Ace.cmp(&Rank::Queen));

        println!("{:?}", Rank::Number(10).cmp(&Rank::Number(2)));

        let hand1 = Hand([
            Rank::Number(4),
            Rank::Number(7),
            Rank::Number(8),
            Rank::Number(9),
            Rank::Number(10),
        ]);
        let hand2 = Hand([
            Rank::Number(6),
            Rank::Number(7),
            Rank::Number(8),
            Rank::Number(9),
            Rank::Number(10),
        ]);

        println!("{:?}", hand1.r#type().cmp(&hand2.r#type()));
        println!("{:?}", hand1.cmp(&hand2));

        let hand3 = Hand::<Rank2>([
            Rank2::Number(2),
            Rank2::Jack,
            Rank2::Number(3),
            Rank2::Number(3),
            Rank2::Number(3),
        ]);

        println!("2333J {:?}", hand3.r#type());

        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Number(2),
            Rank2::Number(3),
        ]);
        println!("JJJ23 {:?}", hand3.r#type());
        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Number(2),
            Rank2::Number(2),
        ]);
        println!("JJJ22 {:?}", hand3.r#type());

        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Number(2),
            Rank2::Number(2),
            Rank2::Number(3),
        ]);

        println!("JJ223 {:?}", hand3.r#type());
        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Number(2),
            Rank2::Number(2),
            Rank2::Number(2),
        ]);

        println!("JJ222 {:?}", hand3.r#type());
        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Jack,
            Rank2::Number(1),
            Rank2::Number(2),
            Rank2::Number(3),
        ]);
        println!("JJ123 {:?}", hand3.r#type());

        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Number(1),
            Rank2::Number(2),
            Rank2::Number(3),
            Rank2::Number(4),
        ]);
        println!("J1234 {:?}", hand3.r#type());

        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Number(1),
            Rank2::Number(2),
            Rank2::Number(4),
            Rank2::Number(4),
        ]);
        println!("J1244 {:?}", hand3.r#type());
        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Number(2),
            Rank2::Number(2),
            Rank2::Number(4),
            Rank2::Number(4),
        ]);
        println!("J2244 {:?}", hand3.r#type());

        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Number(1),
            Rank2::Number(4),
            Rank2::Number(4),
            Rank2::Number(4),
        ]);
        println!("J1444 {:?}", hand3.r#type());

        let hand3 = Hand::<Rank2>([
            Rank2::Jack,
            Rank2::Number(4),
            Rank2::Number(4),
            Rank2::Number(4),
            Rank2::Number(4),
        ]);
        println!("J4444 {:?}", hand3.r#type());
    }

    #[rstest]
    #[case("32T3K")]
    fn test_parsing_hand(#[case] input: &str) {
        println!("{:?}", parse_hand::<Rank>(input).unwrap().1);
    }

    #[rstest]
    #[case("32T3K 765")]
    fn test_parsing_line(#[case] input: &str) {
        println!("{:?}", parse_line::<Rank>(input).unwrap().1);
    }

    #[rstest]
    fn test_parsing_file() {
        let file = include_str!("../test-1.txt");
        let content: Vec<_> = file.lines().map(|line| parse_line::<Rank>(line)).collect();
        println!("{:?}", content);
    }

    #[test]
    fn test_hand_sorting() {
        let fixture = include_str!("../test-1.txt");

        let mut hands: Vec<Hand<Rank>> = fixture
            .lines()
            .map(|line| {
                let (_, (hand, _)) = parse_line(line).unwrap();
                hand
            })
            .collect();
        hands.sort_by(Hand::cmp);
        println!("{:?}", hands);
    }
}
