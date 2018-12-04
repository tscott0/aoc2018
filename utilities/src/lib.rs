use std::fs::File;
use std::io::prelude::*;

pub fn string_from_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn sorted_string_from_file(filename: &str) -> String {
    let contents = string_from_file(filename);

    let mut lines: Vec<&str> = contents.lines().collect();
    lines.sort();

    lines.join("\n")
}
