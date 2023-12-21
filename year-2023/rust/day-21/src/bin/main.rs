use day_21::part1;
use day_21::part2;

fn main() {
    let result = part1::<131>(include_str!("../../input.txt"), 64);

    println!("Part 1: {result:?}");

    let result = part2(include_str!("../../input.txt"), 26501365);

    println!("Part 2: {result:?}");
}