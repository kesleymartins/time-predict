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

        let prediction = self.fuel.predictiction();
        let _result = prediction.predict(&self.fuel);
    }

    fn display(&self) {
        self.fuel.display();
    }
}
