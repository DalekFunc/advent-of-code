use {{crate_name}}::part2;

fn main() {
    let result = part2(include_str!("../../input.txt")).expect("Part 2 failed to run");

    println!("{result}");
}