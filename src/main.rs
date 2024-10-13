#![allow(non_snake_case)]

use std::env;

mod utils;
mod multimedia;
mod inputs_buffer;
mod player;
mod map;
mod tiles;
mod engine;
mod animation;
mod enemy;

use engine::GameEngine;

fn set_resources_path() {
    if let Ok(exec_path) = env::current_exe() {
        if let Some(exec_dir) = exec_path.parent() {
            if exec_dir.ends_with("MacOS") {
                if let Some(contents_dir) = exec_dir.parent() {
                    let resources_dir = contents_dir.join("Resources");
                    if resources_dir.exists() {
                        env::set_current_dir(resources_dir).unwrap();
                    }
                }
            }
        }
    }
}

fn main() {
    set_resources_path();
    let mut gameEngine = GameEngine::Init(1280, 720, 90.0, "assets/map.csv");
    gameEngine.GameLoop();
}
