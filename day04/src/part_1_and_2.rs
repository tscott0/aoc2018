use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Guard {
    id: u32,
    days: HashMap<String, Vec<GuardEvent>>,
}

#[derive(Debug, Copy, Clone)]
struct GuardEvent {
    time: u32,
    event: Event,
}

#[derive(Debug, Copy, Clone)]
enum Event {
    StartShift,
    Sleep,
    Wake,
}

impl Guard {
    fn minutes_asleep(&self) -> u32 {
        println!("Guard #{}", &self.id);
        let mut sleep_sum: u32 = 0;
        for (d, events) in &self.days {
            println!("|- {}", d);
            let mut sleep: Option<u32> = None;
            for e in events {
                if let Some(s) = sleep {
                    match e.event {
                        Event::Sleep => {}
                        Event::Wake => {
                            println!("|--- awake at {}", e.time);

                            sleep_sum += e.time - s;
                            sleep = None;
                        }
                        Event::StartShift => {}
                    }
                } else {
                    match e.event {
                        Event::Sleep => {
                            sleep = Some(e.time);
                            println!("|--- sleep at {}", e.time);
                        }
                        Event::Wake => {}
                        Event::StartShift => {}
                    }
                }
            }
        }
        println!("");
        sleep_sum
    }

    fn most_sleepy_minute(&self) -> u32 {
        let mut sleep_minutes: HashMap<u32, u32> = HashMap::new();
        for (_, events) in &self.days {
            let mut sleep: Option<u32> = None;
            for e in events {
                if let Some(s) = sleep {
                    match e.event {
                        Event::Sleep => {}
                        Event::Wake => {
                            sleep = None;

                            // Record all the minutes spent sleeping
                            for m in s..e.time {
                                sleep_minutes.entry(m).and_modify(|c| *c += 1).or_insert(1);
                            }
                        }
                        Event::StartShift => {}
                    }
                } else {
                    match e.event {
                        Event::Sleep => {
                            sleep = Some(e.time);
                        }
                        Event::Wake => {}
                        Event::StartShift => {}
                    }
                }
            }
        }

        let mut answer: (u32, u32) = (0, 0);
        for (k, v) in sleep_minutes {
            if v > answer.1 {
                answer = (k, v)
            }
        }
        answer.0
    }
}

pub fn solve(input: String) -> (u32, u32) {
    let mut guards: HashMap<u32, Guard> = HashMap::new();

    let line_re = Regex::new(r"\[(\d{4}-\d{2}-\d{2}) \d{2}:(\d{2})\](.*)").unwrap();
    let guard_re = Regex::new(r"Guard \#(\d+)*").unwrap();
    let sleeps_re = Regex::new(r"falls asleep").unwrap();
    let wakes_re = Regex::new(r"wakes up").unwrap();

    let mut guard_id: Option<u32> = None;

    for l in input.lines() {
        let caps = line_re.captures(l).unwrap();

        let day: String = String::from(&caps[1]);
        let time: u32 = caps[2].parse().unwrap();
        let remainder: &str = &caps[3];

        if guard_re.is_match(remainder) {
            let caps = guard_re.captures(l).unwrap();

            let id: u32 = caps[1].parse().unwrap();

            if let Some(x) = guard_id {
                // Changing of the guard \o/
                if x != id {
                    guard_id = Some(id);
                }
            } else {
                // Record the first guard
                guard_id = Some(id);
            }

            guards.entry(id).or_insert(Guard {
                id: id,
                days: HashMap::new(),
            });
        } else if sleeps_re.is_match(remainder) {
            if let Some(id) = guard_id {
                let sleep_event = GuardEvent {
                    time: time,
                    event: Event::Sleep,
                };

                guards.entry(id).and_modify(|g| {
                    g.days
                        .entry(day.clone())
                        .and_modify(|events| {
                            events.push(sleep_event);
                        }).or_insert(vec![GuardEvent {
                            time: time,
                            event: Event::Sleep,
                        }]);
                });
            }
        } else if wakes_re.is_match(remainder) {
            if let Some(id) = guard_id {
                let wake_event = GuardEvent {
                    time: time,
                    event: Event::Wake,
                };

                guards.entry(id).and_modify(|g| {
                    g.days
                        .entry(day)
                        .and_modify(|events| {
                            events.push(wake_event);
                        }).or_insert(vec![GuardEvent {
                            time: time,
                            event: Event::Wake,
                        }]);
                });
            }
        }

        // println!("{} ---- {}", time, remainder);
    }

    let mut sleepiest_guard: Option<(&Guard, u32)> = None;
    for (k, g) in &guards {
        let sleep = g.minutes_asleep();

        if let Some((_, current_most_sleep)) = sleepiest_guard {
            if sleep > current_most_sleep {
                sleepiest_guard = Some((g, sleep));
            }
        } else {
            sleepiest_guard = Some((g, sleep));
        }

        println!("{} {:?}", k, g);
    }

    let mut part_1_answer = 0;
    if let Some((g, _)) = sleepiest_guard {
        let g = g.clone();
        part_1_answer = g.id * g.most_sleepy_minute()
    }

    let mut sleepiest_guard: Option<&Guard> = None;
    let mut part_2_answer = 0;
    for (k, g) in &guards {
        if let Some(guard) = sleepiest_guard {
            if g.most_sleepy_minute() > guard.most_sleepy_minute() {
                sleepiest_guard = Some(g);
                part_2_answer = g.most_sleepy_minute() * g.id;
            }
        } else {
            sleepiest_guard = Some(g);
            part_2_answer = g.most_sleepy_minute() * g.id;
        }
        println!("{} {:?}", k, g);
    }

    (part_1_answer, part_2_answer)
}

#[test]
fn example_1() {
    let input = String::from(
        "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    );

    assert_eq!(solve(input), (240, 4455));
}
