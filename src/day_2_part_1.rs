use std::collections::HashMap;

pub fn solve(input: String) -> i32 {
    let lines = input.split("\n");

    let mut two_count = 0;
    let mut three_count = 0;

    for l in lines {
        let mut word_char_count: HashMap<&str, u32> = HashMap::new();

        let chars = l.split("");
        for c in chars {
            if c != "" {
                word_char_count
                    .entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let mut inc_two = false;
        let mut inc_three = false;

        for (_, value) in word_char_count {
            // println!("{} : {}", k, value);
            if value == 2 {
                inc_two = true;
            } else if value == 3 {
                inc_three = true;
            }
        }

        if inc_two {
            two_count += 1;
        }

        if inc_three {
            three_count += 1;
        }

        // println!("---")
    }

    // println!(
    //     "{} * {} = {}",
    //     two_count,
    //     two_count,
    //     two_count * three_count
    // );
    two_count * three_count
}

#[test]
fn example_1() {
    let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    assert_eq!(12, solve(String::from(input)));
}
