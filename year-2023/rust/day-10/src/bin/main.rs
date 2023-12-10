use day_10::part1;
use day_10::part2;

fn main() {
    let result = part1(include_bytes!("../../input.txt"));

    println!("Part 1: {result:?}");

    let result = part2(include_bytes!("../../input.txt"));

    println!("Part 2: {result:?}");
}
