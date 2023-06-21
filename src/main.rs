mod UTILS;
mod MULTIMEDIA;
mod GAME_ENGINE;
mod INPUTS_BUFFER;
mod PLAYER;
mod MAP;

use crate::GAME_ENGINE::GameEngine;

fn main() {
    let mut gameEngine = GameEngine::New(1280, 720);
    gameEngine.GameLoop();
}


