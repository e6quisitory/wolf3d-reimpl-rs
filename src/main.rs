mod UTILS;
mod multimedia;
mod inputs_buffer;
mod player;
mod map;
mod tiles;
mod game_engine;
mod Map;
mod GAME_ENGINE;

use game_engine::GameEngine;

fn main() {
    let mut gameEngine = GameEngine::Init(1280, 720, 90.0, "map.csv");
    gameEngine.GameLoop();
}