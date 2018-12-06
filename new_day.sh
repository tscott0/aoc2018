#!/bin/bash
set -ex

if [ -z "$1" ]
  then
    echo "No argument supplied"
    exit 1
fi

dirname="day$1"

# Make the new crate
cargo new --bin $dirname

pushd $dirname

touch input

echo "utilities = { path = \"../utilities\" }" >> Cargo.toml

cat << EOF > src/main.rs
mod part_1;
//mod part_2;

extern crate utilities;

fn main() {
    println!(
        "DAY X part 1: {}",
        part_1::solve(utilities::string_from_file("input"))
    );

//    println!(
//        "DAY X part 2: {}",
//        part_2::solve(utilities::string_from_file("input"))
//    );
}
EOF

cat << EOF > src/part_1.rs
pub fn solve(input: String) -> u32 {
    42
}

#[test]
fn example_1() {
    let input = String::from("ni!");

    assert_eq!(solve(input), 99);
}
EOF

popd

# Add the new dir to the vscode workspace and sort dirs
cat aoc.code-workspace \
| jq ".folders += [{ \"path\": \"${dirname}\" }]" \
| jq ".folders |= sort_by(.path)" \
| jq ".folders |= unique" \
> new_workspace

mv new_workspace aoc.code-workspace