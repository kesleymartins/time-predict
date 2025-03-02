use engine::Engine;

mod engine;
mod fuel;
mod prediction;
mod time;

fn main() {
    let mut engine = Engine::new();

    engine.run();
}
