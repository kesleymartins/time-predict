use std::env;

use regex::Regex;

enum Predict {
    OutTime,
    Result,
}

impl Predict {
    fn result(&self, times: &Times) {
        match self {
            Predict::OutTime => {
                println!("calculando o horario de saida.")
            }
            Predict::Result => {
                println!("calculando o saldo feito.")
            }
        }
    }
}

struct Times {
    data: Vec<String>,
}

impl Times {
    fn new(data: Vec<String>) -> Self {
        let mut times = Times { data };

        times.filter_valid_times();
        times.sort_times();

        times
    }

    fn filter_valid_times(&mut self) {
        let reg = Regex::new(r"^([01]?[0-9]|2[0-3]):([0-5]?[0-9])$").unwrap();

        self.data = self
            .data
            .drain(..)
            .filter(|time| reg.is_match(time))
            .collect();
    }

    fn sort_times(&mut self) {
        self.data.sort_by(|cur, next| {
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

    fn predict(&self) -> Predict {
        if self.data.len() % 2 == 0 {
            Predict::Result
        } else {
            Predict::OutTime
        }
    }

    fn display(&self) {
        for (idx, arg) in self.data.iter().enumerate() {
            let kind = if idx % 2 == 0 { "Entrada:" } else { "Saida:  " };

            println!("{} {}", kind, arg);
        }
    }
}

struct Engine {
    times: Times,
}

impl Engine {
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        Self {
            times: Times::new(args),
        }
    }

    fn run(&mut self) {
        self.display();

        let predict = self.times.predict();
        predict.result(&self.times);
    }

    fn display(&self) {
        self.times.display();
    }
}

fn main() {
    let mut engine = Engine::new();

    engine.run();
}
