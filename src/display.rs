use crate::{fuel::Fuel, time::Time};

pub struct Display;

impl Display {
    pub fn logs(fuel: &Fuel) {
        println!("==== Logs ====");

        for time in fuel.iter_times() {
            println!("{}", time.format(false));
        }

        println!();
    }

    pub fn stats(time: &Time) {
        println!(" ==== Logs ====");

        // TODO
    }
}
