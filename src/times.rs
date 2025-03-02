use regex::Regex;

use crate::{predict::Predict, time::Time};

pub struct Times {
    pub data: Vec<(String, Option<Time>)>,
}

impl Times {
    pub fn new(data: Vec<String>) -> Self {
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

    pub fn predict(&self) -> Predict {
        if self.data.len() % 2 == 0 {
            Predict::Result
        } else {
            Predict::OutTime
        }
    }

    pub fn sum(&self) -> Time {
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

    pub fn last_time(&self) -> &Time {
        self.data.last().unwrap().1.as_ref().unwrap()
    }

    pub fn display(&self) {
        for (idx, arg) in self.data.iter().enumerate() {
            let kind = if idx % 2 == 0 { "Entrada:" } else { "Saida:  " };

            println!("{} {}", kind, arg.1.as_ref().unwrap().format(false));
        }

        println!("Tempo realizado: {}", self.sum().format(false));
    }
}
