#[derive(Clone)]
pub struct Time {
    kind: Option<String>,
    sign: i32,
    pub hours: i32,
    pub minutes: i32,
}

impl Time {
    pub fn from_arg(str: &str, pos: usize) -> Self {
        let kind = match pos % 2 {
            0 => ">",
            _ => "<",
        };

        let time_data: Vec<&str> = str.split(":").collect();

        let hours: i32 = time_data[0].parse().unwrap();
        let minutes: i32 = time_data[1].parse().unwrap();

        Self {
            kind: Some(kind.to_string()),
            sign: minutes.signum(),
            hours,
            minutes,
        }
    }

    pub fn from_minutes(min: i32) -> Self {
        let hours = min / 60;
        let minutes = min % 60;

        let sign = minutes.signum();

        Self {
            sign,
            hours: hours.abs(),
            minutes: minutes.abs(),
            kind: None,
        }
    }

    pub fn from_diff(time: &Time, diff: &Time) -> Self {
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
            kind: None,
        }
    }

    pub fn diff(&self, time: &Time) -> Time {
        let minutes = self.in_minutes() - time.in_minutes();

        return Time::from_minutes(minutes);
    }

    pub fn format(&self, with_sign: bool) -> String {
        let mut data: Vec<String> = Vec::new();

        if let Some(kind) = &self.kind {
            data.push(format!("{} ", kind.clone()));
        }

        if with_sign {
            let sign = match self.sign.is_positive() {
                true => "+",
                false => "-",
            };

            data.push(sign.to_string());
        }

        let time = format!("{:0>2}:{:0>2}", self.hours, self.minutes);
        data.push(time);

        let result = data.iter().fold(String::new(), |mut acc, value| {
            acc.push_str(value);

            acc
        });

        result
    }

    pub fn in_minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }
}
