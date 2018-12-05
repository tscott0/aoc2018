pub fn solve(input: String) -> u32 {
    let ascii_lower: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let chars: Vec<char> = input.chars().collect();
    let mut best: u32 = std::u32::MAX;

    for lower in ascii_lower {
        let mut chars_copy = chars.clone();

        strip(&mut chars_copy, lower);

        loop {
            let mut to_remove: Vec<usize> = vec![];
            react(&mut chars_copy, &mut to_remove);

            if to_remove.is_empty() {
                break;
            }
        }

        let score = chars_copy.len() as u32;

        if score < best {
            best = score;
        }

        println!("  {} : {}", lower, score)
    }

    best
}

fn react(chars: &mut Vec<char>, to_remove: &mut Vec<usize>) {
    let mut skip_one = false;

    for i in 0..(chars.len() - 1) {
        let (a, b) = (chars[i], chars[i + 1]);
        if !skip_one && a != b && a.to_string().to_uppercase() == b.to_string().to_uppercase() {
            to_remove.push(i);
            to_remove.push(i + 1);
            skip_one = true;
        } else {
            skip_one = false;
        }
    }

    // Reverse sort the indexes to remove
    to_remove.sort_by(|a, b| b.cmp(a));

    for i in to_remove {
        chars.remove(*i);
    }
}

fn strip(chars: &mut Vec<char>, lower: char) {
    let mut to_remove: Vec<usize> = vec![];

    for i in 0..(chars.len() - 1) {
        if chars[i].to_string().to_uppercase() == lower.to_string().to_uppercase() {
            to_remove.push(i);
        }
    }

    // Reverse sort the indexes to remove
    to_remove.sort_by(|a, b| b.cmp(a));

    for i in to_remove {
        chars.remove(i);
    }
}

#[test]
fn example_1() {
    let input = String::from("dabAcCaCBAcCcaDA");

    assert_eq!(solve(input), 4);
}
