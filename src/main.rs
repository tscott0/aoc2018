mod day_1_part_1;
mod day_1_part_2;
mod day_2_part_1;
mod day_2_part_2;
mod utils;

fn main() {
    println!("Hello, world!");

    println!(
        "DAY 1 part 1: {}",
        day_1_part_1::solve(utils::string_from_file("src/day_1_input.txt"))
    );

    println!(
        "DAY 1 part 2: {}",
        day_1_part_2::solve(utils::string_from_file("src/day_1_input.txt"))
    );

    println!(
        "DAY 2 part 1: {}",
        day_2_part_1::solve(utils::string_from_file("src/day_2_input.txt"))
    );

    println!(
        "DAY 2 part 2: {}",
        day_2_part_2::solve(utils::string_from_file("src/day_2_input.txt"))
    );
}
