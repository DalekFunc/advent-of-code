use day_11::part2;

fn main() {
    let result = part2(include_bytes!("../../input.txt"), 1_000_000).expect("Part 2 failed to run");

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn quick_test() {}
}
