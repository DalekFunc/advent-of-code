use day_23::part1;
use day_23::part2;

fn main() {
    let result = part1::<141>(include_str!("../../input.txt"));

    println!("Part 1: {result:?}");

    let result = part2::<141>(include_str!("../../input.txt"));

    println!("Part 2: {result:?}");
}