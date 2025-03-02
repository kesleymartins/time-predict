#[derive(Clone, Copy)]
pub struct Time {
    pub sign: i32,
    pub hours: i32,
    pub minutes: i32,
}

impl Time {
    pub fn diff(&self, time: &Time) -> Time {
        let minutes = self.in_minutes() - time.in_minutes();

        return Time::from_minutes(minutes);
    }

    pub fn from_minutes(min: i32) -> Self {
        let hours = min / 60;
        let minutes = min % 60;

        let sign = minutes.signum();

        Self {
            sign,
            hours: hours.abs(),
            minutes: minutes.abs(),
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
        }
    }

    fn in_minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    pub fn format(&self, with_wign: bool) -> String {
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
