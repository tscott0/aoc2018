use std::collections::HashSet;

pub fn solve(input: String) -> i32 {
    let mut frequency_set = HashSet::new();
    let mut sum: i32 = 0;
    frequency_set.insert(sum);

    loop {
        let split = input.split("\n");
        for v in split {
            let to_int = v.parse::<i32>().unwrap();

            sum += to_int;

            if frequency_set.contains(&sum) {
                return sum;
            }

            frequency_set.insert(sum);
        }
    }
}

#[test]
fn example_1() {
    assert_eq!(0, solve(String::from("+1\n-1")));
}

#[test]
fn example_2() {
    assert_eq!(10, solve(String::from("+3\n+3\n+4\n-2\n-4")));
}

#[test]
fn example_3() {
    assert_eq!(5, solve(String::from("-6\n+3\n+8\n+5\n-6")));
}

#[test]
fn example_4() {
    assert_eq!(14, solve(String::from("+7\n+7\n-2\n-7\n-4")));
}
