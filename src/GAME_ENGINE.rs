
/*********************************** GAME_ENGINE ***********************************/

use sdl2::video::Window;
use super::MULTIMEDIA::SDLContexts;

pub struct GameEngine {
    pub sdlContexts: SDLContexts,
    pub sdlWindow: Window,
}

impl GameEngine {
    pub fn New(context: SDLContexts) -> Self{
        let sdlWindow = context.CreateWindow("Wolfenstein 3D Clone - Rust", 1280, 720);

        Self {
            sdlContexts: context,
            sdlWindow,
        }
    }
}