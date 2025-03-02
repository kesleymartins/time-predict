use crate::{fuel::Fuel, time::Time};

pub struct Display;

impl Display {
    pub fn logs(fuel: &Fuel) {
        println!("==== Logs ====");

        for (idx, arg) in fuel.times.iter().enumerate() {
            let kind = if idx % 2 == 0 { ">" } else { "<" };

            let time = arg.1.as_ref().unwrap();

            println!("{} {}", kind, time.format(false));
        }

        println!();
    }

    pub fn stats(time: &Time) {
        println!(" ==== Logs ====");

        // TODO
    }
}
