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
        for (pos, arg) in self.args.iter().enumerate() {
            let time = Time::from_arg(arg, pos);

            self.times.push(time);
        }
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

        for (idx, (arg, time)) in paired.iter().enumerate() {
            self.args[idx] = arg.clone();
            self.times[idx] = time.clone();
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
        let mut minutes = 0;

        for chunk in self.times.chunks(2) {
            if chunk.len() == 1 {
                continue;
            }

            let cur = &chunk[0];
            let next = &chunk[1];

            minutes += cur.diff(next).in_minutes();
        }

        Time::from_minutes(minutes)
    }

    pub fn last_check_out(&self) -> Option<Time> {
        let times_len = self.times.len();

        if times_len % 2 == 0 {
            if let Some(time) = self.times.get(times_len - 1) {
                return Some(time.clone());
            }
        }

        if let Some(time) = self.times.get(times_len - 2) {
            return Some(time.clone());
        }

        None
    }
    pub fn last_check_in(&self) -> Option<Time> {
        let times_len = self.times.len();

        if times_len % 2 != 0 {
            if let Some(time) = self.times.get(times_len - 1) {
                return Some(time.clone());
            }
        }

        if let Some(time) = self.times.get(times_len - 2) {
            return Some(time.clone());
        }

        None
    }

    pub fn iter_times(&self) -> std::slice::Iter<Time> {
        self.times.iter()
    }
}
