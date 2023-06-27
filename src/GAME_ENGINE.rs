/*********************************** GAME_ENGINE ***********************************/

use sdl2::{pixels::Color, rect::Rect};
use crate::{
    multimedia::{Multimedia, LightTexture},
    inputs_buffer::InputsBuffer,
    player::Player,
    map::Map,
    UTILS::{
        RAY::Ray,
        DDA::RayCursor
    }, tiles::tileType_t
};

use std::any::Any;

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
        let map = Map::LoadFromCSV(mapCSVPath, &multimedia.assets);
        
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
        self.map.UpdateDoors();
    }

    pub fn RenderFrame(&mut self) {
        self.multimedia.sdlCanvas.clear();

        // Draw ceiling
        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(50, 50, 50, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, 0, self.multimedia.windowParams.windowWidth as u32, (self.multimedia.windowParams.windowHeight/2) as u32)).unwrap();

        // Draw floor
        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(96, 96, 96, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, (self.multimedia.windowParams.windowHeight / 2) as i32, self.multimedia.windowParams.windowWidth as u32, (self.multimedia.windowParams.windowHeight/2) as u32)).unwrap();

        // Raycasting
        for x in 0..=self.multimedia.windowParams.windowWidth-1 {
            let currRay = Ray::New(self.player.position, self.player.viewDir.Rotate(self.multimedia.renderParams.castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, self.player.position);
            let mut prevTile = self.map.GetTile(rayCursor.hitTile).unwrap();
            while self.map.WithinMap(rayCursor.hitTile) {
                let prevTileWasDoor = prevTile.GetTileType() == tileType_t::DOOR;
                rayCursor.GoToNextHit();
                let currTile = self.map.GetTile(rayCursor.hitTile).unwrap();
                prevTile = currTile;
                let currTileResponse = currTile.RayTileHit(&mut rayCursor);

                if currTileResponse.is_none() {
                    continue;
                } else {
                    match currTileResponse.unwrap() {
                        crate::tiles::rayTileHitReturn_t::WALL(textureSliceDistPair) => {
                            
                            // Texture
                            let mut textureSlice = textureSliceDistPair.textureSlice;
                            if prevTileWasDoor {
                                textureSlice.texture = LightTexture(&self.multimedia.assets.gateSideWallTexturePair, rayCursor.GetWallType());
                            }

                            // Screen
                            let distToHitPoint = textureSliceDistPair.dist;
                            let renderHeight = self.multimedia.renderParams.renderHeightProprConst / (distToHitPoint * self.multimedia.renderParams.castingRayAngles[x].1);
                            let screenY = ((self.multimedia.windowParams.windowHeight as f64 / 2.0) - (renderHeight / 2.0)) as i32;
                            let screenRect = Rect::new(x as i32, screenY, 1, renderHeight as u32);

                            // Render onto screen
                            let _ = self.multimedia.sdlCanvas.copy(textureSlice.texture.as_ref(), textureSlice.slice, screenRect);
                                                        
                            break;
                        },
                        crate::tiles::rayTileHitReturn_t::SPRITE(_) => panic!(),
                        crate::tiles::rayTileHitReturn_t::WALL_AND_SPRITES(_) => panic!(),
                        crate::tiles::rayTileHitReturn_t::SPRITES(_) => panic!(),
                    }
                }
            }
        }

        // Refresh screen
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

