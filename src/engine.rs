use std::env;

use crate::fuel::Fuel;

pub struct Engine {
    // Display
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
        self.display();

        let predict = self.fuel.predict();
        predict.result(&self.fuel);
    }

    fn display(&self) {
        self.fuel.display();
    }
}
