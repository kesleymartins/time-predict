use crate::{fuel::Fuel, stats::Stats};

pub struct Display;

impl Display {
    pub fn logs(fuel: &Fuel) {
        println!("==== Logs ====");

        for time in fuel.iter_times() {
            println!("{}", time.format(false));
        }

        println!();
    }

    pub fn stats(stats: &Stats) {
        println!("==== Logs ====");

        if let Some(out_time) = &stats.out_time {
            println!("saida prevista: {}", out_time.format(false));
        } else {
            println!("saida prevista: -");
        }

        if let Some(real_out_time) = &stats.real_out_time {
            println!("saida real: {}", real_out_time.format(false));
        } else {
            println!("saida real: -");
        }

        println!("saldo: {}", stats.balance.format(true));
    }
}
