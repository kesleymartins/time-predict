use crate::{fuel::Fuel, time::Time};

pub enum Prediction {
    OutTime,
    Result,
}

impl Prediction {
    pub fn predict(&self, fuel: &Fuel) -> Time {
        match self {
            Prediction::OutTime => self.predict_out_time(&fuel),
            Prediction::Result => self.predict_result(&fuel),
        }
    }

    fn predict_out_time(&self, fuel: &Fuel) -> Time {
        let last_time = fuel.last_time();
        let work_time = Time {
            sign: 1,
            hours: 8,
            minutes: 48,
        };

        let time_sum = fuel.sum();
        let time_diff = time_sum.diff(&work_time);

        let next_time = Time::from_diff(&last_time, &time_diff);

        next_time
    }

    fn predict_result(&self, times: &Fuel) -> Time {
        let work_time = Time {
            sign: 1,
            hours: 8,
            minutes: 48,
        };

        let time_sum = times.sum();
        let time_diff = time_sum.diff(&work_time);

        time_diff
    }
}
