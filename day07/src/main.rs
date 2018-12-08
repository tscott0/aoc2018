mod part_1;
//mod part_2;

extern crate regex;
extern crate utilities;

fn main() {
    println!(
        "DAY 7 part 1: {}",
        part_1::solve(utilities::string_from_file("input"))
    );

    //    println!(
    //        "DAY 7 part 2: {}",
    //        part_2::solve(utilities::string_from_file("input"))
    //    );
}
