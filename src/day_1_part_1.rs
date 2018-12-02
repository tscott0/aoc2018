pub fn solve(input: String) -> i32 {
    let split = input.split("\n");
    let mut sum: i32 = 0;

    for v in split {
        let to_int = v.parse::<i32>().unwrap();
        sum += to_int;
    }

    sum
}

#[test]
fn example_1() {
    assert_eq!(3, solve(String::from("+1\n+1\n+1")));
}

#[test]
fn example_2() {
    assert_eq!(0, solve(String::from("+1\n+1\n-2")));
}

#[test]
fn example_3() {
    assert_eq!(-6, solve(String::from("-1\n-2\n-3")));
}
