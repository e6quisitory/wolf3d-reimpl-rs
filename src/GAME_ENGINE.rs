/*********************************** GAME_ENGINE ***********************************/

use sdl2::{pixels::Color, rect::Rect};
use crate::{
    MULTIMEDIA::{Multimedia},
    INPUTS_BUFFER::InputsBuffer,
    PLAYER::Player,
    MAP::Map,
    UTILS::{
        RAY::Ray,
        DDA::{ RayCursor, wallType_t }
    }
};

pub struct GameEngine {
    pub multimedia: Multimedia,
    pub inputsBuffer: InputsBuffer,
    pub player: Player,
    pub map: Map
}

impl GameEngine {
    pub fn Init(windowWidth: usize, windowHeight: usize, fov: f64, mapCSVPath: &str) -> Self {
        let multimedia = Multimedia::New(windowWidth, windowHeight, fov);
        let inputsBuffer = InputsBuffer::default();
        let player = Player::New();
        let map = Map::LoadFromCSV(mapCSVPath);
        
        Self {
            multimedia,
            inputsBuffer,
            player,
            map
        }
    }

    pub fn Update(&mut self) {
        self.inputsBuffer.Update(&mut self.multimedia.sdlEventPump);
        self.player.Update(&self.inputsBuffer, &self.map);
    }

    pub fn RenderFrame(&mut self) {
        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.multimedia.sdlCanvas.clear();

        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(50, 50, 50, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, 0, self.multimedia.windowParams.windowWidth as u32, (self.multimedia.windowParams.windowHeight/2) as u32)).unwrap();

        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(96, 96, 96, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, (self.multimedia.windowParams.windowHeight / 2) as i32, self.multimedia.windowParams.windowWidth as u32, (self.multimedia.windowParams.windowHeight/2) as u32)).unwrap();

        for x in 0..self.multimedia.windowParams.windowWidth -1 {
            let currRay = Ray::New(self.player.position, self.player.viewDir.Rotate(self.multimedia.renderParams.castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, self.player.position);
            while self.map.WithinMap(rayCursor.hitTile) {
                rayCursor.GoToNextHit();
                let textureID = self.map.GetTile(rayCursor.hitTile);
                if textureID != 0 {
                    let dist = rayCursor.GetDistToHitPoint();

                    let propr_const = 1.15 * (self.multimedia.windowParams.windowWidth as f64) / ((16.0 / 9.0) * (self.multimedia.renderParams.fov / 72.0));

                    let renderHeight = propr_const / (dist * self.multimedia.renderParams.castingRayAngles[x].1);
                    if rayCursor.GetWallType() == wallType_t::VERTICAL {
                        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(199, 199, 199, 255));
                    } else {
                        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(81, 81, 81, 255));
                    }

                    //canvas.fill_rect(Rect::new(x as i32, 0, 1, 10));
                    let y = ((self.multimedia.windowParams.windowHeight as f64 / 2.0) - (renderHeight / 2.0)) as i32;
                    //sdlCanvas.fill_rect(Rect::new(x as i32, y, 1, renderHeight as u32)).unwrap();
                    let _ = self.multimedia.sdlCanvas.copy(&self.multimedia.assets.wallTextures[textureID as usize], Rect::new((rayCursor.GetWidthPercent() as f64 * 64.0) as i32, 0, 1, 64),Rect::new(x as i32, y, 1, renderHeight as u32));
                    break;
                }
            }
        }
        self.multimedia.sdlCanvas.present();
    }

    pub fn GameLoop(&mut self) {
        loop {
            self.Update();
            if self.inputsBuffer.quit { break; }
            self.RenderFrame();
        }
    }
}

