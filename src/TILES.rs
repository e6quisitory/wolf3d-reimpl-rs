/*********************************** TILES ***********************************/

use std::rc::Rc;

use sdl2::{render::Texture, rect::Rect};
use crate::{utils::{vec2d::{Point2, Vec2}, dda::RayCursor, conventions::TEXTURE_PITCH}, multimedia::LightTexture};

/**************** Types ****************/

#[derive(Clone)]
pub struct TextureSlice {
    pub textureID: i32,
    pub slice: Rect
}

#[derive(Clone)]
pub struct TextureSliceDistPair {
    pub textureSlice: TextureSlice,
    pub dist: f64
}

#[derive(Clone)]
pub struct SpriteRenderData {
    pub location: Point2,
    pub textureID: i32
}

#[derive(Clone)]
pub struct TexturePair {
    pub litTextureID: i32,
    pub unlitTextureID: i32
}

#[derive(Clone)]
pub enum Tile {
    WALL(Wall),
    DOOR(Door),
    EMPTY(EmptyTile),
    OBJECT(Object),
    COLLECTIBLE(Collectible),
    NONE
}

/**************** Wall ****************/

#[derive(Clone)]
pub struct Wall {
    pub texturePair: TexturePair
}

impl Wall {
    pub fn New(litTextureID: i32, unlitTextureID: i32) -> Self {
        Self {
            texturePair: TexturePair {
                litTextureID,
                unlitTextureID,
            }
        }
    }

    pub fn RayTileHit(&self, rayCursor: &mut RayCursor) -> TextureSliceDistPair {
        let widthPercent = rayCursor.GetWidthPercent();
        let textureX = (widthPercent * TEXTURE_PITCH as f64) as i32;
        let textureID = LightTexture(rayCursor, &self.texturePair);

        let textureSlice = TextureSlice {
            textureID,
            slice: Rect::new(textureX, 0, 1, TEXTURE_PITCH)
        };

        let textureSliceDistPair = TextureSliceDistPair {
            textureSlice,
            dist: rayCursor.GetDistToHitPoint()
        };

        return textureSliceDistPair;
    }

    pub fn PlayerTileHit() -> bool {
        return true;
    }
}

/**************** Door ****************/

#[derive(Clone, PartialEq)]
pub enum DoorStatus {
    OPEN,
    CLOSED,
    OPENING,
    CLOSING
}

#[derive(Clone)]
pub enum DoorPosition {
    OPEN = 0,
    CLOSED = 1
}

#[derive(Clone)]
pub enum DoorTimerVal {
    NO_TIME_LEFT   = 0,
    FULL_TIME_LEFT = 1
}

#[derive(Clone)]
pub struct Door {
    pub position: f64,
    pub status: DoorStatus,
    pub timerVal: f64,
    pub enemiesWithin: Vec<Enemy>,
    pub spriteRenderDataList: Vec<SpriteRenderData>,
}

impl Door {
    pub fn New() -> Self {
        Self {
            position: DoorPosition::CLOSED as i32 as f64,
            status: DoorStatus::CLOSED,
            timerVal: DoorTimerVal::FULL_TIME_LEFT as i32 as f64,
            enemiesWithin: Vec::new(),
            spriteRenderDataList: Vec::new()
        }
    }

    pub fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<TextureSliceDistPair> {
        // Center hit point
        let mut centeredHitInfo = rayCursor.GetNextCenterHit();

        // First check if incoming ray actually intersects with middle of tile (the gate)
        if centeredHitInfo.hitTile == rayCursor.hitTile {

            let centerWidthPercent = centeredHitInfo.GetWidthPercent();

            // Ray does intersect gate, but now check if the gate *blocks* the ray
            if centerWidthPercent < self.position {

                // If ray is blocked by gate, then output the proper gate texture and rect
                let gateTexturePair = TexturePair {litTextureID: 99, unlitTextureID: 100};
                let gateWidthPercent = self.position - centerWidthPercent;

                let gateTextureX = (gateWidthPercent* TEXTURE_PITCH as f64) as i32;
                let gateTextureSlice = TextureSlice {
                    textureID: LightTexture(rayCursor, &gateTexturePair),
                    slice: Rect::new(gateTextureX, 0, 1, TEXTURE_PITCH)
                };
                let gateDistance = centeredHitInfo.GetDistToHitPoint();
                let gateTextureSliceDistPair = TextureSliceDistPair {
                    textureSlice: gateTextureSlice,
                    dist: gateDistance,
                };

                return Some(gateTextureSliceDistPair);
            } else {
                // Ray is not blocked by gate, meaning it passes through the DoorTile entirely
                return None;
            }

        } else {
            // Ray does not intersect with middle of tile ==> it hits sidewall
            // Let it pass through ; renderer will detect sidewall hit and swap texture accordingly
            return None;
        }
    }

    pub fn PlayerTileHit(&self) -> bool {
        if self.position < 0.2 {
            false
        } else {
            true
        }
    }

    pub fn Update(&mut self, moveIncr: f64, timerIncr: f64, playerInsideDoor: bool) {    
        match self.status {
            DoorStatus::OPEN => {
                if !playerInsideDoor {
                    self.timerVal -= timerIncr;
                    if self.timerVal < 0.0 {
                        self.status = DoorStatus::CLOSING;
                    }
                } else {
                    self.timerVal = 1.0;
                }
            },
            DoorStatus::OPENING => {
                self.position -= moveIncr;
                if self.position < 0.0 {
                    self.position = 0.0;
                    self.status = DoorStatus::OPEN;
                    self.timerVal = 1.0;
                }
            },
            DoorStatus::CLOSING => {
                self.position += moveIncr;
                if self.position > 1.0 {
                    self.position = 1.0;
                    self.status = DoorStatus::CLOSED;
                }
            },
            DoorStatus::CLOSED => {}
        }
    }
}

/**************** EmptyTile ****************/

#[derive(Clone)]
pub struct EmptyTile {
    pub enemiesWithin: Vec<Enemy>,
    pub spriteRenderDataList: Vec<SpriteRenderData>,
}

impl EmptyTile {
    pub fn New() -> Self {
        Self {
            enemiesWithin: Vec::new(),
            spriteRenderDataList: Vec::new()
        }
    }

    pub fn RayTileHit() -> Option<Vec<SpriteRenderData>> {
        return None;
    }

    pub fn PlayerTileHit() -> bool {
        return false;
    }
}

/**************** Object ****************/

#[derive(Clone)]
pub struct Object {
    pub texture: Rc<Texture>
}

/**************** Collectible ****************/

#[derive(Clone)]
pub struct Collectible {
    pub texture: Rc<Texture>,
    pub collected: bool
}

/**************** Enemy ****************/

#[derive(Clone)]
pub struct Enemy {
    pub position: Point2,
    pub viewDir: Vec2,
    pub texture: Rc<Texture>
}