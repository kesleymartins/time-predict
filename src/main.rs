use std::env;

use regex::Regex;

struct Times;

impl Times {
    fn filter(data: Vec<String>) -> Vec<String> {
        let reg = Regex::new(r"^([01]?[0-9]|2[0-3]):([0-5]?[0-9])$").unwrap();

        data.into_iter().filter(|time| reg.is_match(time)).collect()
    }

    fn fix_order(data: &mut Vec<String>) {
        data.sort_by(|cur, next| {
            let cur: Vec<&str> = cur.split(":").collect();
            let next: Vec<&str> = next.split(":").collect();

            let cur_hours: i32 = cur[0].parse().unwrap();
            let cur_minutes: i32 = cur[1].parse().unwrap();

            let next_hours: i32 = next[0].parse().unwrap();
            let next_minutes: i32 = next[1].parse().unwrap();

            if cur_hours == next_hours {
                cur_minutes.cmp(&next_minutes)
            } else {
                cur_hours.cmp(&next_hours)
            }
        });
    }
}

struct Engine {
    times: Vec<String>,
}

impl Engine {
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut times = Times::filter(args);
        Times::fix_order(&mut times);

        Self { times }
    }

    fn display(&self) {
        for (idx, arg) in self.times.iter().enumerate() {
            let kind = if idx % 2 == 0 { "Entrada:" } else { "Saida:  " };

            println!("{} {}", kind, arg);
        }
    }
}

fn main() {
    let args = Engine::new();

    args.display();
}
