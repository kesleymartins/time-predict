use engine::Engine;

mod engine;
mod fuel;
mod predict;
mod time;

fn main() {
    let mut engine = Engine::new();

    engine.run();
}
