pub fn solve(input: String) -> String {
    let lines: Vec<&str> = input.split("\n").collect();

    for l1 in &lines {
        for l2 in &lines {
            match out_by_one_answer(String::from(*l1), String::from(*l2)) {
                Some(answer) => return answer,
                None => continue,
            }
        }
    }
    String::from("")
}

fn out_by_one_answer(id_one: String, id_two: String) -> Option<String> {
    if id_one.len() != id_two.len() {
        return None::<String>;
    }

    let mut possible_answer = String::from("");
    let mut difference_count = 0;
    for i in 0..id_one.len() {
        if id_one.chars().nth(i) != id_two.chars().nth(i) {
            difference_count += 1;
            if difference_count > 1 {
                return None::<String>;
            }
        } else {
            possible_answer.push(id_one.chars().nth(i).unwrap());
        }
    }

    if difference_count == 1 {
        return Some(possible_answer);
    }

    None
}

#[test]
fn example_1() {
    let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    assert_eq!("fgij", solve(String::from(input)));
}
