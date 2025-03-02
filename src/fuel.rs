use regex::Regex;

use crate::{prediction::Prediction, time::Time};

pub struct Fuel {
    args: Vec<String>,
    times: Vec<Time>,
}

impl Fuel {
    pub fn new(args: Vec<String>) -> Self {
        let mut fuel = Fuel {
            args,
            times: Vec::new(),
        };

        fuel.filter_valid_times();
        fuel.parse_args();
        fuel.sort_times();

        fuel
    }

    fn filter_valid_times(&mut self) {
        let reg = Regex::new(r"^([01]?[0-9]|2[0-3]):([0-5]?[0-9])$").unwrap();

        self.args = self
            .args
            .drain(..)
            .filter(|arg| reg.is_match(&arg))
            .collect();
    }

    fn parse_args(&mut self) {
        self.times = self
            .args
            .iter()
            .map(|arg| {
                let time_data: Vec<&str> = arg.split(":").collect();

                let hours: i32 = time_data[0].parse().unwrap();
                let minutes: i32 = time_data[1].parse().unwrap();

                Time::from_minutes(hours * 60 + minutes)
            })
            .collect();
    }

    fn sort_times(&mut self) {
        let mut paired: Vec<(String, Time)> = self
            .args
            .iter()
            .cloned()
            .zip(self.times.iter().cloned())
            .collect();

        paired.sort_by(|cur, next| {
            if cur.1.hours == next.1.hours {
                cur.1.minutes.cmp(&next.1.minutes)
            } else {
                cur.1.hours.cmp(&next.1.hours)
            }
        });

        // Desempacota de volta para os arrays
        for (i, (v1, v2)) in paired.iter().enumerate() {
            self.args[i] = v1.clone();
            self.times[i] = *v2;
        }
    }

    pub fn predictiction(&self) -> Prediction {
        if self.args.len() % 2 == 0 {
            Prediction::Result
        } else {
            Prediction::OutTime
        }
    }

    pub fn sum(&self) -> Time {
        let mut hours = 0;
        let mut minutes = 0;

        for chunk in self.times.chunks(2) {
            if chunk.len() == 1 {
                continue;
            }

            let cur = &chunk[0];
            let next = &chunk[1];

            hours += next.hours - cur.hours;
            minutes = next.minutes - cur.minutes;
        }

        while minutes > 60 {
            hours += 1;
            minutes -= 60;
        }

        Time {
            sign: minutes.signum(),
            hours,
            minutes,
        }
    }

    pub fn last_time(&self) -> &Time {
        self.times.last().as_ref().unwrap()
    }

    pub fn iter_times(&self) -> std::slice::Iter<Time> {
        self.times.iter()
    }
}
