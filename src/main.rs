#![allow(non_snake_case)]

mod utils;
mod multimedia;
mod inputs_buffer;
mod player;
mod map;
mod tiles;
mod engine;

use engine::GameEngine;

fn main() {
    let mut gameEngine = GameEngine::Init(1440, 900, 90.0, "assets/map.csv");
    gameEngine.GameLoop();
}