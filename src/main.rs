mod utils;
mod multimedia;
mod inputs_buffer;
mod player;
mod map;
mod tiles;
mod engine;

use engine::GameEngine;

fn main() {
    let mut gameEngine = GameEngine::Init(960, 540, 90.0, "map.csv");
    gameEngine.GameLoop();
}