use day_8::part1;
use day_8::part2;

fn main() {
    let result = part1(include_str!("../../input.txt"));

    println!("Part 1: {result:?}");

    let result = part2(include_str!("../../input.txt"));

    println!("Part 2: {result:?}");
}
