use std::env;

use crate::{display::Display, fuel::Fuel};

pub struct Engine {
    // Tags
    fuel: Fuel,
}

impl Engine {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        Self {
            fuel: Fuel::new(args),
        }
    }

    pub fn run(&mut self) {
        let prediction = self.fuel.predictiction();
        let result = prediction.predict(&self.fuel);

        Display::logs(&self.fuel);
        Display::stats(&result);
    }
}
