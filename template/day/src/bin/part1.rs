use {{crate_name}}::part1;

fn main() {
    let result = part1(include_str!("../../input.txt")).expect("Part 1 failed to run");

    println!("{result}");
}