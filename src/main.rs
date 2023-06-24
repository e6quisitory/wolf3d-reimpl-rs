mod UTILS;
mod MULTIMEDIA;
mod INPUTS_BUFFER;
mod PLAYER;
mod MAP;
mod GAME_ENGINE;

use GAME_ENGINE::GameEngine;

fn main() {
    let mut gameEngine = GameEngine::Init(1280, 720, 90.0, "map.csv");
    gameEngine.GameLoop();
}