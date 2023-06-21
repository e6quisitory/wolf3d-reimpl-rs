
/*********************************** GAME_ENGINE ***********************************/

use sdl2::video::Window;
use crate::PLAYER::Player;
use super::MULTIMEDIA::SDLContexts;
use super::INPUTS_BUFFER::InputsBuffer;

pub struct GameEngine {
    pub sdlContexts: SDLContexts,
    pub sdlWindow: Window,
    pub inputsBuffer: InputsBuffer,
    pub player: Player
}

impl GameEngine {
    pub fn New(sdlContexts: SDLContexts) -> Self{
        let sdlWindow = sdlContexts.CreateWindow("Wolfenstein 3D Clone - Rust", 1280, 720);
        let inputsBuffer = InputsBuffer::default();
        let player = Player::New();

        Self {
            sdlContexts,
            sdlWindow,
            inputsBuffer,
            player
        }
    }
}