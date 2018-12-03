use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: String) -> u32 {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    //  0   1 2  3 4
    // #1 @ 1,3: 4x4
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for l in input.lines() {
        let caps = re.captures(l).unwrap();

        let x: u32 = caps[2].parse().unwrap();
        let y: u32 = caps[3].parse().unwrap();
        let w: u32 = caps[4].parse().unwrap();
        let h: u32 = caps[5].parse().unwrap();

        // println!("n:\"{}\" x:{} y:{} w:{} h:{}", &caps[0], x, y, w, h);

        for i in x..(x + w) {
            // println!("X:{}", i);
            for j in y..(y + h) {
                // println!(" Y:{}", j);

                map.entry((j, i))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    // for ((x, y), v) in map.iter() {
    //     println!("{},{} count: {}", x, y, v)
    // }

    map.iter().filter(|(_, &v)| v >= 2).count() as u32
}

#[test]
fn example_1() {
    let input = String::from(
        "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
    );

    assert_eq!(solve(input), 4);
}
