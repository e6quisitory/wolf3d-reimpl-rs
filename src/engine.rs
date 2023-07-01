use core::panic;

use sdl2::{pixels::Color, rect::Rect};
use crate::{
    multimedia::{Multimedia, LightTexture, TextureType},
    inputs_buffer::InputsBuffer,
    player::Player,
    map::Map,
    utils::{
        ray::Ray,
        dda::RayCursor, vec2d::Dot, conventions::TEXTURE_PITCH
    }, tiles::{Tile, TextureHandle, Sprite, WallSlice}
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
    playerSwivelIncr: f64,

    // Render related
    wallSlicesBuffer: Vec<(i32, WallSlice)>,
    spritesBuffer: Vec<Sprite>,
    wallRenderHeights: Vec<i32>,
    spriteTileHitMap: Vec<Vec<bool>>
}

impl GameEngine {
    pub fn Init(windowWidth: usize, windowHeight: usize, fov: f64, mapCSVPath: &str) -> Self {
        let multimedia = Multimedia::New(windowWidth, windowHeight, fov);
        let inputsBuffer = InputsBuffer{windowLock: true, ..Default::default()};
        let player = Player::New();
        let map = Map::LoadFromCSV(mapCSVPath);
        
        let refreshRatePropr = multimedia.displayParams.refreshRate as f64 / 60.0;
        let doorMoveIncr = 0.02/refreshRatePropr;
        let doorTimerIncr = 0.01/refreshRatePropr;
        let playerMoveIncr = 0.1/refreshRatePropr;
        let playerSwivelIncr = 0.00125/refreshRatePropr;

        let wallRenderHeights: Vec<i32> = vec![0; multimedia.windowParams.width];

        let spriteTileHitMap: Vec<Vec<bool>> = vec![vec![false; map.height as usize]; map.width as usize];

        Self {
            multimedia,
            inputsBuffer,
            player,
            map,

            doorMoveIncr,
            doorTimerIncr,
            playerMoveIncr,
            playerSwivelIncr,

            wallSlicesBuffer: Vec::new(),
            spritesBuffer: Vec::new(),
            wallRenderHeights,

            spriteTileHitMap
        }
    }

    pub fn GameLoop(&mut self) {
        loop {
            self.Update();
            if self.inputsBuffer.quit { break; }
            self.RenderFrame();
        }
    }

    fn Update(&mut self) {
        self.inputsBuffer.Update(&mut self.multimedia.sdlContexts.sdlContext, &mut self.multimedia.sdlEventPump);
        self.player.Update(&self.inputsBuffer, &mut self.map, self.playerMoveIncr, self.playerSwivelIncr);
        self.map.UpdateDoors(self.doorMoveIncr, self.doorTimerIncr, self.player.location);
    }

    fn RenderFrame(&mut self) {
        self.multimedia.sdlCanvas.clear();
        self.DrawCeilingAndFloor();
        self.RenderIntoBuffers();
        self.DrawWallsFromBuffer();
        self.DrawSpritesFromBuffer();
        self.multimedia.sdlCanvas.present();
    }

    fn DrawCeilingAndFloor(&mut self) {
        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(50, 50, 50, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, 0, self.multimedia.windowParams.width as u32, (self.multimedia.windowParams.height/2) as u32)).unwrap();

        self.multimedia.sdlCanvas.set_draw_color(Color::RGBA(96, 96, 96, 255));
        self.multimedia.sdlCanvas.fill_rect(Rect::new(0, (self.multimedia.windowParams.height / 2) as i32, self.multimedia.windowParams.width as u32, (self.multimedia.windowParams.height/2) as u32)).unwrap();
    }

    fn RenderIntoBuffers(&mut self) {
        self.wallSlicesBuffer.clear();
        self.spritesBuffer.clear();
        self.ResetSpriteTileHitMap();

        for x in 0..self.multimedia.windowParams.width {
            let currRay = Ray::New(self.player.location, self.player.viewDir.Rotate(self.multimedia.renderParams.castingRayAngles[x].0));
            let mut rayCursor = RayCursor::New(currRay, self.player.location);
            let mut prevTile = self.map.GetTile(rayCursor.hitTile);
            while self.map.WithinMap(rayCursor.hitTile) {
                let prevTileWasDoor = if let Tile::DOOR(_) = prevTile { true } else { false };
                rayCursor.GoToNextHit();
                let currTile = self.map.GetTile(rayCursor.hitTile);
                prevTile = currTile;

                match currTile {
                    Tile::WALL(wall) => {
                        let mut wallSlice = wall.GetWallSlice(&mut rayCursor);
                        if prevTileWasDoor {
                            let gateSidewall_lit = TextureHandle::New(TextureType::WALL, 101);
                            let gateSideWall_unlit = TextureHandle::New(TextureType::WALL, 102);
                            wallSlice.textureHandle = LightTexture(&mut rayCursor, gateSidewall_lit, gateSideWall_unlit);
                        }
                        self.wallSlicesBuffer.push((x as i32, wallSlice));
                        break;
                    },
                    Tile::DOOR(door) => {
                        let doorWallSlice = door.GetWallSlice(&mut rayCursor);
                        if doorWallSlice.is_some() {
                            self.wallSlicesBuffer.push((x as i32, doorWallSlice.unwrap()));
                            break;
                        } else {
                            continue;
                        }
                    },
                    Tile::EMPTY(empty) => {
                        let currTileX = rayCursor.hitTile.x() as usize;
                        let currTileY = rayCursor.hitTile.y() as usize;
                        if !self.spriteTileHitMap[currTileX][currTileY] {
                            let spritesArr = empty.GetSprites();
                            if spritesArr.is_some() {
                                for sprite in spritesArr.unwrap() {
                                    self.spritesBuffer.push(*sprite);
                                }
                            }
                            self.spriteTileHitMap[currTileX][currTileY] = true;
                        }
                        continue;
                    },
                    Tile::NONE => panic!(),
                };
            }
        }
    }

    fn DrawWallsFromBuffer(&mut self) {
        self.ResetWallRenderHeights();
        
        for w in &self.wallSlicesBuffer {
            let x = w.0;
            let wallSlice = &w.1;
            
            let distToHitPoint = wallSlice.dist;
            let renderHeight = self.multimedia.renderParams.renderHeightProprConst / (distToHitPoint * self.multimedia.renderParams.castingRayAngles[x as usize].1);
            let screenY = ((self.multimedia.windowParams.height as f64 / 2.0) - (renderHeight / 2.0)) as i32;
            let screenRect = Rect::new(x as i32, screenY, 1, renderHeight as u32);
            self.wallRenderHeights[x as usize] = renderHeight as i32;

            let texture = self.multimedia.assets.GetTexture(wallSlice.textureHandle);

            // Render onto screen
            let _ = self.multimedia.sdlCanvas.copy(texture, wallSlice.textureRect, screenRect);
        }
    }

    fn DrawSpritesFromBuffer(&mut self) {
        self.spritesBuffer.reverse();
        for sprite in &self.spritesBuffer {
            let vecToSprite = sprite.location - self.player.location;
            let spriteHitDistY = Dot(vecToSprite, self.player.viewDir);
            let spriteHitDistX = Dot(vecToSprite, self.player.east);
            let screenX = ((self.multimedia.windowParams.width/2) as f64 + ((self.multimedia.renderParams.projPlaneDist/spriteHitDistY)*spriteHitDistX)) as i32;
            let renderHeight = self.multimedia.renderParams.renderHeightProprConst / spriteHitDistY;

            let spriteScreenRect = Rect::new(screenX - (renderHeight/2.0) as i32, (self.multimedia.windowParams.height as i32)/2 - (renderHeight/2.0) as i32, renderHeight as u32, renderHeight as u32);
        
            for x in spriteScreenRect.x..(spriteScreenRect.x+spriteScreenRect.w) {
                if x < 0 {
                    continue;
                } else if x >= self.multimedia.windowParams.width as i32 {
                    break;
                } else {
                    if spriteScreenRect.h >= self.wallRenderHeights[x as usize] {
                        let spriteTextureWidthPercent = (x - spriteScreenRect.x) as f64 / (spriteScreenRect.w) as f64;
                        let spriteTextureX = (spriteTextureWidthPercent * TEXTURE_PITCH as f64) as i32;
                        let spriteTextureRect = Rect::new(spriteTextureX, 0, 1, TEXTURE_PITCH);
                        let screenRect = Rect::new(x, spriteScreenRect.y, 1, spriteScreenRect.h as u32);
                        
                        let texture = self.multimedia.assets.GetTexture(sprite.textureHandle);

                        let _ = self.multimedia.sdlCanvas.copy(texture, spriteTextureRect, screenRect);
                    }
                }
            }
        }
    }

    fn ResetSpriteTileHitMap(&mut self) {
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                self.spriteTileHitMap[x as usize][y as usize] = false;
            }
        }
    }

    fn ResetWallRenderHeights(&mut self) {
        for i in 0..self.wallRenderHeights.len() {
            self.wallRenderHeights[i] = 0;
        }
    }
}

