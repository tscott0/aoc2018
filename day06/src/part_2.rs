use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }

    fn manhattan_distance_to(&self, x: i64, y: i64) -> i64 {
        let x_diff: i64 = if self.x > x { self.x - x } else { x - self.x };
        let y_diff: i64 = if self.y > y { self.y - y } else { y - self.y };

        x_diff + y_diff
    }
}

pub fn solve(input: String, max_distance: u32) -> u32 {
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let mut locs: HashMap<Coord, u32> = HashMap::new();

    let mut min_x: i64 = std::i64::MAX;
    let mut max_x: i64 = 0;
    let mut min_y: i64 = std::i64::MAX;
    let mut max_y: i64 = 0;

    let margin: i64 = 1000;
    let debug: bool = false;

    for l in input.lines() {
        let caps = re.captures(l).unwrap();

        let x: i64 = caps[1].parse().unwrap();
        let y: i64 = caps[2].parse().unwrap();

        locs.entry(Coord::new(x, y)).or_insert(0);

        if x < min_x {
            min_x = x;
        }

        if x > max_x {
            max_x = x;
        }

        if y < min_y {
            min_y = y;
        }

        if y > max_y {
            max_y = y;
        }
    }

    if debug {
        println!(
            "min_x:{} max_x:{} min_y:{} max_y:{}",
            min_x, max_x, min_y, max_y
        );
    }

    let mut total_cells: u32 = 0;

    for y in min_y - margin..=max_y + margin {
        for x in min_x - margin..=max_x + margin {
            let mut distance_sum: u32 = 0;

            for (&c, _) in &locs {
                distance_sum += c.manhattan_distance_to(x, y) as u32;
            }

            if distance_sum < max_distance {
                total_cells += 1;
            }
        }
    }

    total_cells
}

#[test]
fn example_1() {
    let input = String::from(
        "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
    );

    assert_eq!(solve(input, 32), 16);
}
