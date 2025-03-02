use std::env;

use crate::times::Times;

pub struct Engine {
    // Display
    // Tags
    times: Times,
}

impl Engine {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        Self {
            times: Times::new(args),
        }
    }

    pub fn run(&mut self) {
        self.display();

        let predict = self.times.predict();
        predict.result(&self.times);
    }

    fn display(&self) {
        self.times.display();
    }
}
