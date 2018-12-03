use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: String) -> u32 {
    let mut map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut claims: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    //  0   1 2  3 4
    // #1 @ 1,3: 4x4
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for l in input.lines() {
        let caps = re.captures(l).unwrap();

        let claim: u32 = caps[1].parse().unwrap();
        let x: u32 = caps[2].parse().unwrap();
        let y: u32 = caps[3].parse().unwrap();
        let w: u32 = caps[4].parse().unwrap();
        let h: u32 = caps[5].parse().unwrap();

        for i in x..(x + w) {
            for j in y..(y + h) {
                map.entry((j, i))
                    .and_modify(|vec| vec.push(claim))
                    .or_insert(vec![claim]);

                claims
                    .entry(claim)
                    .and_modify(|vec| vec.push((j, i)))
                    .or_insert(vec![(j, i)]);
            }
        }
    }

    // Filter the map so it only has squares with only one claim
    let single_claims: HashMap<(u32, u32), Vec<u32>> = map
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(&k, v)| (k, v.clone()))
        .collect();

    for (claim, coords) in claims {
        let mut winner = true;
        for c in coords {
            if !single_claims.contains_key(&c) {
                winner = false;
            }
        }

        if winner {
            return claim;
        }
    }

    0 // Possibly should return Result<T, E> instead
}
