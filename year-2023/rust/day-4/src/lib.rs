use anyhow::Result;
use parsing::Card;

pub mod parsing;

pub fn part1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|line| line.parse::<Card>().unwrap().point())
        .sum())
}

pub fn part2(input: &str) -> Result<u32> {
    // id, score, amount
    let mut book: Vec<(u32, u32, u32)> = input
        .lines()
        .map(|line| {
            let card = line.parse::<Card>().expect("msg");
            (card.id, card.matching(), 1)
        })
        .collect();

    let og_total = book.len();

    for n in 1..og_total {
        let (prev, curr) = book.split_at_mut(n);

        prev.iter().for_each(|(id, score, amount)| {
            // when the card has non-zero score
            // and score + card number >= n
            if *score != 0 && (id + score) >= (n + 1) as u32 {
                curr[0].2 += amount;
            }
        })
    }

    Ok(book.into_iter().map(|t| t.2).sum())
}
