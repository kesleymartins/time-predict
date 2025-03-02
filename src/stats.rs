use crate::time::Time;

pub struct Stats {
    pub balance: Time,
    pub out_time: Option<Time>,
    pub real_out_time: Option<Time>,
}

impl Stats {
    pub fn new(balance: Time, out_time: Option<Time>, real_out_time: Option<Time>) -> Self {
        Self {
            balance,
            out_time,
            real_out_time,
        }
    }
}
