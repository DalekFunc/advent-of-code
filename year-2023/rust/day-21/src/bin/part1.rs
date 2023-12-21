use day_21::part1;

fn main() {
    let result = part1::<131>(include_str!("../../input.txt"), 64).expect("Part 1 failed to run");

    println!("{result}");
}