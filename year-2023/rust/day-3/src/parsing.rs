use std::collections::HashMap;

fn insert_number(
    to: &mut HashMap<(Coordinates, Coordinates), u32>,
    start: usize,
    end: usize,
    line_number: usize,
    digits: &[char],
) {
    to.insert(
        ((start, line_number), (end, line_number)),
        digits.iter().collect::<String>().parse().unwrap(),
    );
}

pub fn parse_line(
    numbers: &mut HashMap<(Coordinates, Coordinates), u32>,
    symbols: &mut HashMap<Coordinates, char>,
    input: &str,
    line_number: usize,
) {
    let iter = input.chars().enumerate();
    let mut storing_number = false;
    let mut digits_collected: Vec<char> = vec![];
    let mut number_starting = 0;

    for (pos, c) in iter {
        match c {
            '.' => {
                if storing_number {
                    insert_number(
                        numbers,
                        number_starting,
                        pos - 1,
                        line_number,
                        &digits_collected,
                    );
                    storing_number = false;
                    digits_collected.drain(..);
                }
            }
            c if "+-*/%#@$&=".contains(c) => {
                if storing_number {
                    insert_number(
                        numbers,
                        number_starting,
                        pos - 1,
                        line_number,
                        &digits_collected,
                    );
                    storing_number = false;
                    digits_collected.drain(..);
                }

                symbols.insert((pos, line_number), c);
            }

            c if c.is_ascii_digit() => {
                if !storing_number {
                    storing_number = true;
                    number_starting = pos;
                }
                digits_collected.push(c);
            }
            c => {
                panic!("Unexpected characters.: {}", c);
            }
        }
    }

    // when number is at the end
    if storing_number {
        insert_number(
            numbers,
            number_starting,
            input.len() - 1,
            line_number,
            &digits_collected,
        );
    }
}

pub type Coordinates = (usize, usize);

pub fn has_adjacent_symbols(
    symbols: &mut HashMap<Coordinates, char>,
    start: &Coordinates,
    end: &Coordinates,
) -> bool {
    // check head
    if start.0 != 0 && symbols.contains_key(&((start.0 - 1), start.1)) {
        return true;
    }

    // check tail
    if symbols.contains_key(&(end.0 + 1, start.1)) {
        return true;
    }

    // check prev line
    if start.1 != 0
        && (start.0.saturating_sub(1)..=(end.0 + 1))
            .any(|x| symbols.contains_key(&(x, start.1 - 1)))
    {
        return true;
    }

    // check next line
    if (start.0.saturating_sub(1)..=(end.0 + 1)).any(|x| symbols.contains_key(&(x, start.1 + 1))) {
        return true;
    }

    false
}

pub fn is_adjacent_to(start: &Coordinates, end: &Coordinates, symbol_pos: Coordinates) -> bool {
    // check head
    if start.0 != 0 && symbol_pos == (start.0 - 1, start.1) {
        return true;
    }

    // check tail
    if symbol_pos == (end.0 + 1, start.1) {
        return true;
    }
    // check top and bottom
    if start.1 != 0
        && symbol_pos.1 == start.1 - 1
        && (start.0.saturating_sub(1)..=end.0 + 1).contains(&symbol_pos.0)
    {
        return true;
    }
    if symbol_pos.1 == start.1 + 1
        && (start.0.saturating_sub(1)..=end.0 + 1).contains(&symbol_pos.0)
    {
        return true;
    }

    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_numbers() {
        let fixture = "467..114..";
        let mut numbers: HashMap<(Coordinates, Coordinates), u32> = HashMap::new();
        let mut symbols: HashMap<Coordinates, char> = HashMap::new();

        parse_line(&mut numbers, &mut symbols, &fixture, 0);

        assert!(symbols.is_empty());
        assert!(numbers.contains_key(&((0, 0), (2, 0))));
        assert!(numbers.contains_key(&((5, 0), (7, 0))));
    }

    #[test]
    fn test_parsing_numbers_at_the_end() {
        let fixture = ".......592";
        let mut numbers: HashMap<(Coordinates, Coordinates), u32> = HashMap::new();
        let mut symbols: HashMap<Coordinates, char> = HashMap::new();

        parse_line(&mut numbers, &mut symbols, &fixture, 0);

        assert!(numbers.contains_key(&((7, 0), (9, 0))));
    }

    #[test]
    fn test_parsing_symbols() {
        let fixture = "617*......";
        let mut numbers: HashMap<(Coordinates, Coordinates), u32> = HashMap::new();
        let mut symbols: HashMap<Coordinates, char> = HashMap::new();

        parse_line(&mut numbers, &mut symbols, &fixture, 0);

        assert!(numbers.contains_key(&((0, 0), (2, 0))));
        assert!(symbols.contains_key(&(3, 0)));
    }
}
