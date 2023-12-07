// region:    --- Modules

use std::collections::HashMap;

use anyhow::Result;
use parsing::{has_adjacent_symbols, is_adjacent_to, parse_line, Coordinates};

pub mod parsing;

// endregion: --- Modules

pub fn part1(input: &str) -> Result<u32> {
    let mut numbers: HashMap<(Coordinates, Coordinates), u32> = HashMap::new();
    let mut symbols: HashMap<Coordinates, char> = HashMap::new();

    input.lines().enumerate().for_each(|(line_number, line)| {
        parse_line(&mut numbers, &mut symbols, line, line_number);
    });

    Ok(numbers
        .iter()
        .filter(|((start, end), _)| has_adjacent_symbols(&mut symbols, start, end))
        .map(|(_, val)| *val)
        .sum())
}

pub fn part2(input: &str) -> Result<u32> {
    let mut numbers: HashMap<(Coordinates, Coordinates), u32> = HashMap::new();
    let mut symbols: HashMap<Coordinates, char> = HashMap::new();

    input.lines().enumerate().for_each(|(line_number, line)| {
        parse_line(&mut numbers, &mut symbols, line, line_number);
    });

    let gears: Vec<_> = symbols
        .iter()
        .filter(|(_, c)| **c == '*')
        .filter(|(pos, _)| {
            numbers
                .iter()
                .filter(|((start, end), _)| is_adjacent_to(start, end, **pos))
                .count()
                == 2
        })
        .collect();

    Ok(gears
        .iter()
        .map(|(pos, _)| {
            numbers
                .iter()
                .filter_map(|((start, end), value)| {
                    if is_adjacent_to(start, end, **pos) {
                        Some(value)
                    } else {
                        None
                    }
                })
                .product::<u32>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_head() {
        let fixture = r#"
.....
@123.
.....
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }

    #[test]
    fn test_tail() {
        let fixture = r#"
.....
.123@
.....
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }

    #[test]
    fn test_top_leading_diagonal() {
        let fixture = r#"
@....
.123.
.....
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }
    #[test]
    fn test_top_trailing_diagonal() {
        let fixture = r#"
....@
.123.
.....
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }

    #[test]
    fn test_bottom_leading_diagonal() {
        let fixture = r#"
.....
.123.
@....
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }
    #[test]
    fn test_bottom_trailing_diagonal() {
        let fixture = r#"
.....
.123.
....@
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }

    #[test]
    fn test_top() {
        let fixture = r#"
.@...
.123.
.....
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }

    #[test]
    fn test_bottom() {
        let fixture = r#"
.....
.123.
..@..
"#;
        assert_eq!(123, part1(fixture).unwrap());
    }
}
