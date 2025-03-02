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
                let last_time = times.last_time();
                let work_time = Time {
                    sign: 1,
                    hours: 8,
                    minutes: 48,
                };

                let time_sum = times.sum();
                let time_diff = time_sum.diff(&work_time);

                let next_time = Time::from_diff(&last_time, &time_diff);

                println!("Horário previsto: {}", next_time.format(false));
            }

            Predict::Result => {
                let work_time = Time {
                    sign: 1,
                    hours: 8,
                    minutes: 48,
                };

                let time_sum = times.sum();
                let time_diff = time_sum.diff(&work_time);

                println!("Saldo realizado: {}", time_diff.format(true));
            }
        }
    }
}

struct Time {
    sign: i32,
    hours: i32,
    minutes: i32,
}

impl Time {
    fn diff(&self, time: &Time) -> Time {
        let minutes = self.in_minutes() - time.in_minutes();

        return Time::from_minutes(minutes);
    }

    fn from_minutes(min: i32) -> Self {
        let hours = min / 60;
        let minutes = min % 60;

        let sign = minutes.signum();

        Self {
            sign,
            hours: hours.abs(),
            minutes: minutes.abs(),
        }
    }

    fn from_diff(time: &Time, diff: &Time) -> Self {
        let mut hours = time.hours + diff.hours;
        let mut minutes = time.minutes + diff.minutes;

        if minutes >= 60 {
            hours += 1;
            minutes -= 60;
        }

        let sign = minutes.signum();

        Self {
            sign,
            hours,
            minutes,
        }
    }

    fn in_minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    fn format(&self, with_wign: bool) -> String {
        if with_wign {
            let sign = match self.sign.is_positive() {
                true => "+",
                false => "-",
            };

            return format!("{}{:0>2}:{:0>2}", sign, self.hours, self.minutes);
        }

        format!("{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

struct Times {
    data: Vec<(String, Option<Time>)>,
}

impl Times {
    fn new(data: Vec<String>) -> Self {
        let data = data.iter().map(|arg| (arg.clone(), None)).collect();

        let mut times = Times { data };

        times.filter_valid_times();
        times.parse_data();
        times.sort_times();

        times
    }

    fn filter_valid_times(&mut self) {
        let reg = Regex::new(r"^([01]?[0-9]|2[0-3]):([0-5]?[0-9])$").unwrap();

        self.data = self
            .data
            .drain(..)
            .filter(|time| reg.is_match(&time.0))
            .collect();
    }

    fn parse_data(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|time| {
                let time_data: Vec<&str> = time.0.split(":").collect();

                let hours: i32 = time_data[0].parse().unwrap();
                let minutes: i32 = time_data[1].parse().unwrap();

                let new_time = Time::from_minutes(hours * 60 + minutes);

                (time.0.clone(), Some(new_time))
            })
            .collect();
    }

    fn sort_times(&mut self) {
        self.data.sort_by(|cur, next| {
            let cur = cur.1.as_ref().unwrap();
            let next = next.1.as_ref().unwrap();

            if cur.hours == next.hours {
                cur.minutes.cmp(&next.minutes)
            } else {
                cur.hours.cmp(&next.hours)
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

    fn sum(&self) -> Time {
        let mut hours = 0;
        let mut minutes = 0;

        for chunk in self.data.chunks(2) {
            if chunk.len() == 1 {
                continue;
            }

            let cur = &chunk[0].1.as_ref().unwrap();
            let next = &chunk[1].1.as_ref().unwrap();

            hours += next.hours - cur.hours;
            minutes = next.minutes - cur.minutes;
        }

        while minutes > 60 {
            hours += 1;
            minutes -= 60;
        }

        let sign = minutes.signum();

        Time {
            sign,
            hours,
            minutes,
        }
    }

    fn last_time(&self) -> &Time {
        self.data.last().unwrap().1.as_ref().unwrap()
    }

    fn display(&self) {
        for (idx, arg) in self.data.iter().enumerate() {
            let kind = if idx % 2 == 0 { "Entrada:" } else { "Saida:  " };

            println!("{} {}", kind, arg.1.as_ref().unwrap().format(false));
        }

        println!("Tempo realizado: {}", self.sum().format(false));
    }
}

struct Engine {
    // Display
    // Tags
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
