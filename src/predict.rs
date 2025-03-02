use crate::{Times, time::Time};

pub enum Predict {
    OutTime,
    Result,
}

impl Predict {
    pub fn result(&self, times: &Times) {
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
