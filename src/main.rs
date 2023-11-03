#![allow(non_snake_case)]

mod utils;
mod multimedia;
mod inputs_buffer;
mod player;
mod map;
mod tiles;
mod engine;
mod animation;

use engine::GameEngine;

fn main() {
    let mut gameEngine = GameEngine::Init(1280, 720, 90.0, "assets/map.csv");
    gameEngine.GameLoop();
}
