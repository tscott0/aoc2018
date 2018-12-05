pub fn solve(input: String) -> u32 {
    let mut chars: Vec<char> = input.chars().collect();

    loop {
        let mut to_remove: Vec<usize> = vec![];

        react(&mut chars, &mut to_remove);

        if to_remove.is_empty() {
            break;
        }

        // Reverse sort the indexes to remove
        to_remove.sort_by(|a, b| b.cmp(a));

        for i in to_remove {
            // println!("Removing {}={}", i, chars[i]);

            chars.remove(i);
        }
    }

    chars.len() as u32
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
}

#[test]
fn example_1() {
    let input = String::from("dabAcCaCBAcCcaDA");

    assert_eq!(solve(input), 10);
}
