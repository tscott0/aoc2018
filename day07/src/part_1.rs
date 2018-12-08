use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: String) -> String {
    //Step C must be finished before step A can begin.
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let mut map: HashMap<char, Vec<char>> = HashMap::new();

    for l in input.lines() {
        let caps = re.captures(l).unwrap();

        let a: char = caps[1].parse().unwrap();
        let b: char = caps[2].parse().unwrap();

        println!("{} -> {}", a, b);

        map.entry(a).and_modify(|x| x.push(b)).or_insert(vec![b]);
    }

    // Possibly find first char to start with

    let mut answer: HashSet<char> = HashSet::new();

    loop {
        match next(&map, &answer) {
            Some(c) => answer.insert(c),
            None => break,
        };
    }

    String::from("BANANA")
}

fn next(map: &HashMap<char, Vec<char>>, exclude: &HashSet<char>) -> Option<char> {
    let mut possible_next: Vec<char> = vec![];

    for (c, children) in map {
        if !exclude.contains(c) {
            for child in children {
                if !exclude.contains(child) {
                    possible_next.push(*child);
                }
            }
        }
    }

    Some('c')
}

#[test]
fn example_1() {
    let input = String::from(
        "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
    );

    assert_eq!(solve(input), String::from("CABDFE"));
}
