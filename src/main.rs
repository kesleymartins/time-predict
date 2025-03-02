use engine::Engine;

mod display;
mod engine;
mod fuel;
mod prediction;
mod time;

fn main() {
    let mut engine = Engine::new();

    engine.run();
}
