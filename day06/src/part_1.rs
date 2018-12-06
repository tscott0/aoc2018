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

pub fn solve(input: String) -> u32 {
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let mut locs: HashMap<Coord, u32> = HashMap::new();

    let mut min_x: i64 = std::i64::MAX;
    let mut max_x: i64 = 0;
    let mut min_y: i64 = std::i64::MAX;
    let mut max_y: i64 = 0;

    let esc_char = vec![27];
    let esc = String::from_utf8(esc_char).unwrap();
    let reset: u8 = 0;
    let bright: u8 = 1;
    let black: u8 = 30;
    let red: u8 = 31;
    let green: u8 = 32;
    let mut current_colour: u8 = reset;

    let margin: i64 = 1;
    let threshold: u32 = 10000;

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

    println!(
        "min_x:{} max_x:{} min_y:{} max_y:{}",
        min_x, max_x, min_y, max_y
    );

    for y in min_y - margin..=max_y + margin {
        for x in min_x - margin..=max_x + margin {
            let mut closest: Option<(Coord, i64, bool)> = None;
            for (&c, _) in &locs {
                let distance: i64 = c.manhattan_distance_to(x, y);

                if let Some((r, d, tie)) = closest {
                    if distance < d {
                        // println!("distance: {}", distance);
                        closest = Some((c, distance, false));
                    } else if distance == d {
                        // TIE
                        // println!("{},{} : {:?} and {:?} are {} away", x, y, c, r, d);
                        closest = Some((c, distance, true));
                    }
                } else {
                    closest = Some((c, distance, false));
                }
            }

            if let Some((c, _, tie)) = closest {
                // println!("closest to {:?}", c);
                if !tie {
                    locs.entry(c).and_modify(|x| *x += 1);
                    current_colour = green;
                } else {
                    current_colour = black;
                }
            }

            if locs.contains_key(&Coord::new(x, y)) {
                // print!("@");
                current_colour = red;
                print!(
                    "{}[{};{}m{}{}[{}m",
                    esc, bright, current_colour, "@", esc, reset
                );
            } else {
                // print!(".");
                print!(
                    "{}[{};{}m{}{}[{}m",
                    esc, bright, current_colour, ".", esc, reset
                );
            }
        }
        print!("\n")
    }

    let mut current_best: u32 = 0;
    for l in &locs {
        if l.1 < &threshold && *l.1 > current_best {
            current_best = *l.1;
        }
        println!("{:?} = {}", l.0, l.1);
    }

    // let test = Coord::new(1, 2);
    // let x: i64 = -1;
    // let y: i64 = -1;
    // println!(
    //     "{:?} mh distance to {},{} = {}",
    //     test,
    //     1,
    //     1,
    //     test.manhattan_distance_to(x, y)
    // );

    current_best
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

    assert_eq!(solve(input), 17);
}
