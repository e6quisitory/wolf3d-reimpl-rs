
/*********************************** GAME_ENGINE ***********************************/

use ndarray::SliceInfoElem::Index;
use sdl2::video::Window;
use super::MULTIMEDIA::SDLContexts;
use super::INPUTS_BUFFER::InputsBuffer;

pub struct GameEngine {
    pub sdlContexts: SDLContexts,
    pub sdlWindow: Window,
    pub inputsBuffer: InputsBuffer
}

impl GameEngine {
    pub fn New(sdlContexts: SDLContexts) -> Self{
        let sdlWindow = sdlContexts.CreateWindow("Wolfenstein 3D Clone - Rust", 1280, 720);
        let inputsBuffer = InputsBuffer::default();

        Self {
            sdlContexts,
            sdlWindow,
            inputsBuffer
        }
    }
}