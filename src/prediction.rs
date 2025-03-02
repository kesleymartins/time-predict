use crate::{fuel::Fuel, stats::Stats, time::Time};

pub enum Prediction {
    OutTime,
    Result,
}

impl Prediction {
    pub fn predict(&self, fuel: &Fuel) -> Stats {
        match self {
            Prediction::OutTime => self.predict_out_time(&fuel),
            Prediction::Result => self.predict_result(&fuel),
        }
    }

    fn predict_out_time(&self, fuel: &Fuel) -> Stats {
        let work_time = Time::from_minutes(8 * 60 + 48);
        let worked_time = fuel.sum();

        let last_check_in = fuel.last_check_in().unwrap();

        let balance = worked_time.diff(&work_time);
        let out_time = Some(Time::from_diff(&last_check_in, &balance));
        let real_out_time = None;

        Stats::new(balance, out_time, real_out_time)
    }

    fn predict_result(&self, fuel: &Fuel) -> Stats {
        let work_time = Time::from_minutes(8 * 60 + 48);
        let worked_time = fuel.sum();

        let balance = worked_time.diff(&work_time);
        let out_time = None;
        let real_out_time = fuel.last_check_out();

        Stats::new(balance, out_time, real_out_time)
    }
}
