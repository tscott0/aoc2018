mod part_1_and_2;

extern crate regex;
extern crate utilities;

fn main() {
    let (part_1, part_2) = part_1_and_2::solve(utilities::sorted_string_from_file("input"));
    println!("DAY 4 part 1: {}\nDAY 4 part 2: {}", part_1, part_2);
}
