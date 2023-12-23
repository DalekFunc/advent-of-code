use day_14::part1;
use day_14::part2;

fn main() {
    let result = part1(include_bytes!("../../input.txt"));

    println!("Part 1: {result:?}");

    let result = part2(include_bytes!("../../input.txt"));
    // let result = part2(include_bytes!("../../test-1.txt"));

    println!("Part 2: {result:?}");
}
