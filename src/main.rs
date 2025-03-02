use engine::Engine;
use times::Times;

mod engine;
mod predict;
mod time;
mod times;

fn main() {
    let mut engine = Engine::new();

    engine.run();
}
