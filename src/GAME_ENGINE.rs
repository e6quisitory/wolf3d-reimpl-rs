
/*********************************** GAME_ENGINE ***********************************/

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::MAP::Map;
use crate::MULTIMEDIA::{RenderParams, WindowParams};
use crate::PLAYER::Player;
use crate::UTILS::DDA::{RayCursor, wallType_t};
use crate::UTILS::RAY::Ray;
use super::MULTIMEDIA::SDLContexts;
use super::INPUTS_BUFFER::InputsBuffer;

pub struct GameEngine {
    pub sdlContexts: SDLContexts,
    pub sdlCanvas: WindowCanvas,
    pub sdlEventPump: EventPump,
    pub windowParams: WindowParams,
    pub inputsBuffer: InputsBuffer,
    pub player: Player,
    pub map: Map,
    pub renderParams: RenderParams
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
        let sdlEventPump = sdlContexts.sdlContext.event_pump().unwrap();
        let windowParams = WindowParams { windowWidth, windowHeight };
        let inputsBuffer = InputsBuffer::default();
        let player = Player::New();
        let map = Map::LoadFromCSV("map.csv");
        let renderParams = RenderParams::New(90.0, windowWidth);

        Self {
            sdlContexts,
            sdlCanvas,
            sdlEventPump,
            windowParams,
            inputsBuffer,
            player,
            map,
            renderParams
        }
    }

    pub fn Update(&mut self) {
        self.inputsBuffer.Update(&mut self.sdlEventPump);
        self.player.Update(&self.inputsBuffer);
    }

    pub fn RenderFrame(&mut self) {
        self.sdlCanvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.sdlCanvas.clear();

        for x in 0..self.windowParams.windowWidth -1 {
            let currRay = Ray::New(self.player.position, self.player.viewDir.Rotate(self.renderParams.castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, self.player.position);
            while self.map.WithinMap(rayCursor.hitTile) {
                rayCursor.GoToNextHit();
                if self.map.GetTile(rayCursor.hitTile) == 1 {
                    let dist = rayCursor.GetDistToHitPoint();
                    let renderHeight = 400.0/(dist*self.renderParams.castingRayAngles[x].1);
                    if rayCursor.GetWallType() == wallType_t::VERTICAL {
                        self.sdlCanvas.set_draw_color(Color::RGBA(199, 199, 199, 255));
                    } else {
                        self.sdlCanvas.set_draw_color(Color::RGBA(81, 81, 81, 255));
                    }

                    //canvas.fill_rect(Rect::new(x as i32, 0, 1, 10));
                    let y = ((self.windowParams.windowHeight as f64 / 2.0) - (renderHeight / 2.0)) as i32;
                    self.sdlCanvas.fill_rect(Rect::new(x as i32, y, 1, renderHeight as u32)).unwrap();
                    break;
                }
            }
        }

        self.sdlCanvas.present();
    }

    pub fn GameLoop(&mut self) {
        loop {
            self.Update();

            if self.inputsBuffer.quit {
                break;
            }

            self.RenderFrame();
        }
    }
}