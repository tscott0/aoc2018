mod day_1_part_1;
mod day_1_part_2;
mod day_2_part_1;
mod day_2_part_2;
mod day_3_part_1;
mod day_3_part_2;
mod utils;

extern crate regex;

fn main() {
    println!("Hello, world!");

    println!(
        "DAY 1 part 1: {}",
        day_1_part_1::solve(utils::string_from_file("src/input/day_1"))
    );

    println!(
        "DAY 1 part 2: {}",
        day_1_part_2::solve(utils::string_from_file("src/input/day_1"))
    );

    println!(
        "DAY 2 part 1: {}",
        day_2_part_1::solve(utils::string_from_file("src/input/day_2"))
    );

    println!(
        "DAY 2 part 2: {}",
        day_2_part_2::solve(utils::string_from_file("src/input/day_2"))
    );

    println!(
        "DAY 3 part 1: {}",
        day_3_part_1::solve(utils::string_from_file("src/input/day_3"))
    );

    println!(
        "DAY 3 part 2: {}",
        day_3_part_2::solve(utils::string_from_file("src/input/day_3"))
    );
}
