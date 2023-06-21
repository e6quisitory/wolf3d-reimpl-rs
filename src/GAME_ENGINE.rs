
/*********************************** GAME_ENGINE ***********************************/

use sdl2::render::WindowCanvas;
use crate::MAP::Map;
use crate::MULTIMEDIA::WindowParams;
use crate::PLAYER::Player;
use super::MULTIMEDIA::SDLContexts;
use super::INPUTS_BUFFER::InputsBuffer;

pub struct GameEngine {
    pub sdlContexts: SDLContexts,
    pub sdlCanvas: WindowCanvas,
    pub windowParams: WindowParams,
    pub inputsBuffer: InputsBuffer,
    pub player: Player,
    pub map: Map
}

impl GameEngine {
    pub fn New(windowWidth: usize, windowHeight: usize) -> Self {
        let sdlContexts = SDLContexts::New();
        let sdlCanvas = sdlContexts
            .CreateWindow("Wolfenstein 3D Clone - Rust", windowWidth as u32, windowHeight as u32)
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        let windowParams = WindowParams { windowWidth, windowHeight };
        let inputsBuffer = InputsBuffer::default();
        let player = Player::New();
        let map = Map::LoadFromCSV("map.csv");

        Self {
            sdlContexts,
            sdlCanvas,
            windowParams,
            inputsBuffer,
            player,
            map
        }
    }
}