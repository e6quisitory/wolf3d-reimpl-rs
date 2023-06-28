/*********************************** GAME_ENGINE ***********************************/

use sdl2::{pixels::Color, rect::Rect};
use crate::{
    multimedia::{Multimedia, LightTexture, TextureType},
    inputs_buffer::InputsBuffer,
    player::Player,
    map::Map,
    utils::{
        ray::Ray,
        dda::RayCursor
    }, tiles::Tile
};

pub struct GameEngine {
    pub multimedia: Multimedia,
    pub inputsBuffer: InputsBuffer,
    pub player: Player,
    pub map: Map,

    // Time related
    doorMoveIncr: f64,
    doorTimerIncr: f64,
    playerMoveIncr: f64,
    playerSwivelIncr: f64

}

impl GameEngine {
    pub fn Init(windowWidth: usize, windowHeight: usize, fov: f64, mapCSVPath: &str) -> Self {
        let multimedia = Multimedia::New(windowWidth, windowHeight, fov);
        let inputsBuffer = InputsBuffer::default();
        let player = Player::New();
        let map = Map::LoadFromCSV(mapCSVPath);
        
        let refreshRatePropr = multimedia.displayParams.refreshRate as f64 / 60.0;
        let doorMoveIncr = 0.02/refreshRatePropr;
        let doorTimerIncr = 0.01/refreshRatePropr;
        let playerMoveIncr = 0.1/refreshRatePropr;
        let playerSwivelIncr = 0.00125/refreshRatePropr;

        Self {
            multimedia,
            inputsBuffer,
            player,
            map,

            doorMoveIncr,
            doorTimerIncr,
            playerMoveIncr,
            playerSwivelIncr
        }
    }

    pub fn Update(&mut self) {
        self.inputsBuffer.Update(&mut self.multimedia.sdlEventPump);
        self.player.Update(&self.inputsBuffer, &mut self.map, self.playerMoveIncr, self.playerSwivelIncr);
        self.map.UpdateDoors(self.doorMoveIncr, self.doorTimerIncr, self.player.position);
    }

    pub fn RenderFrame(&mut self) {
        self.multimedia.sdlCanvas.clear();

        // Draw ceiling
        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(50, 50, 50, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, 0, self.multimedia.windowParams.width as u32, (self.multimedia.windowParams.height/2) as u32)).unwrap();

        // Draw floor
        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(96, 96, 96, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, (self.multimedia.windowParams.height / 2) as i32, self.multimedia.windowParams.width as u32, (self.multimedia.windowParams.height/2) as u32)).unwrap();

        // Raycasting
        for x in 0..=self.multimedia.windowParams.width-1 {
            let currRay = Ray::New(self.player.position, self.player.viewDir.Rotate(self.multimedia.renderParams.castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, self.player.position);
            let mut prevTile = self.map.GetTile(rayCursor.hitTile);
            while self.map.WithinMap(rayCursor.hitTile) {
                let prevTileWasDoor = if let Tile::DOOR(_) = prevTile { true } else { false };
                rayCursor.GoToNextHit();
                let currTile = self.map.GetTile(rayCursor.hitTile);
                prevTile = currTile;

                let currTileResponse = match currTile {
                    Tile::WALL(wall) => Some(wall.RayTileHit(&mut rayCursor)),
                    Tile::DOOR(door) => door.RayTileHit(&mut rayCursor),
                    Tile::EMPTY(_) => None,
                    Tile::OBJECT(_) => panic!(),
                    Tile::COLLECTIBLE(_) => panic!(),
                    Tile::NONE => panic!(),
                };

                if currTileResponse.is_none() {
                    continue;
                } else {
                    let textureSliceDistPair = currTileResponse.unwrap();

                    // Texture
                    let textureSlice = textureSliceDistPair.textureSlice;
                    let texture = self.multimedia.assets.GetTexture(TextureType::WALL, {
                        if prevTileWasDoor {
                            LightTexture(&mut rayCursor, &self.multimedia.assets.gateSidewallTexturePair)
                        } else {
                            textureSlice.textureID
                        }
                    });

                    // Screen
                    let distToHitPoint = textureSliceDistPair.dist;
                    let renderHeight = self.multimedia.renderParams.renderHeightProprConst / (distToHitPoint * self.multimedia.renderParams.castingRayAngles[x].1);
                    let screenY = ((self.multimedia.windowParams.height as f64 / 2.0) - (renderHeight / 2.0)) as i32;
                    let screenRect = Rect::new(x as i32, screenY, 1, renderHeight as u32);

                    // Render onto screen
                    let _ = self.multimedia.sdlCanvas.copy(texture, textureSlice.slice, screenRect);

                    break;
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

