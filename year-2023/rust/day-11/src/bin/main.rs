use day_11::part1;
use day_11::part2;

fn main() {
    let result = part1(include_bytes!("../../input.txt"));

    println!("Part 1: {result:?}");

    let result = part2(include_bytes!("../../input.txt"), 1_000_000);

    println!("Part 2: {result:?}");
}
