use core::panic;

use sdl2::rect::Rect;
use crate::{utils::{dda::RayCursor, conventions::TEXTURE_PITCH, vec2d::Point2}, multimedia::{LightTexture, TextureType}};

/**************************************************************** Types ****************************************************************/

#[derive(Clone)]
pub enum Tile {
    WALL(Wall),
    DOOR(Door),
    OBJECT(ObjectTile),
    EMPTY(EmptyTile),
    NONE
}

#[derive(Copy, Clone)]
pub struct TextureHandle {
    pub textureType: TextureType,
    pub ID: i32
}

impl TextureHandle {
    pub fn New(textureType: TextureType, ID: i32) -> Self {
        Self {
            textureType,
            ID
        }
    }
}

pub struct WallSlice {
    pub textureHandle: TextureHandle,
    pub textureRect: Rect,
    pub dist: f64
}

#[derive(Copy, Clone, PartialEq)]
pub enum SpriteType {
    OBJECT,
    ENEMY
}

#[derive(Copy, Clone)]
pub struct Sprite {
    pub spriteType: SpriteType,
    pub textureHandle: TextureHandle,
    pub location: Point2
}

/**************************************************************** Wall ****************************************************************/

#[derive(Clone)]
pub struct Wall {
    litTextureHandle: TextureHandle,
    unlitTextureHandle: TextureHandle
}

impl Wall {
    pub fn New(litTextureID: i32, unlitTextureID: i32) -> Self {
        Self {
            litTextureHandle:   TextureHandle::New(TextureType::WALL, litTextureID),
            unlitTextureHandle: TextureHandle::New(TextureType::WALL, unlitTextureID)
        }
    }

    pub fn GetWallSlice(&self, rayCursor: &mut RayCursor) -> WallSlice {
        let textureHandle = LightTexture(rayCursor, self.litTextureHandle, self.unlitTextureHandle);
        let widthPercent = rayCursor.GetWidthPercent();
        let textureX = (widthPercent * TEXTURE_PITCH as f64) as i32;
        let textureRect = Rect::new(textureX, 0, 1, TEXTURE_PITCH);
        let dist = rayCursor.GetDistToHitPoint();

        WallSlice {
            textureHandle,
            textureRect,
            dist
        }
    }

    pub fn PlayerTileHit() -> bool {
        return true;
    }
}

/**************************************************************** Door ****************************************************************/

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
}

impl Door {
    pub fn New() -> Self {
        Self {
            position: DoorPosition::CLOSED as i32 as f64,
            status: DoorStatus::CLOSED,
            timerVal: DoorTimerVal::FULL_TIME_LEFT as i32 as f64,
        }
    }

    pub fn GetWallSlice(&self, rayCursor: &mut RayCursor) -> Option<WallSlice> {
        // Center hit point
        let mut centeredHitInfo = rayCursor.GetNextCenterHit();

        // First check if incoming ray actually intersects with middle of tile (the gate)
        if centeredHitInfo.hitTile == rayCursor.hitTile {

            let centerWidthPercent = centeredHitInfo.GetWidthPercent();

            // Ray does intersect gate, but now check if the gate *blocks* the ray
            if centerWidthPercent < self.position {

                // If ray is blocked by gate, then output the proper gate texture and rect
                let litGateTexture = TextureHandle::New(TextureType::WALL, 99);
                let unlitGateTexture = TextureHandle::New(TextureType::WALL, 100);

                let gateWidthPercent = self.position - centerWidthPercent;
                let gateTextureX = (gateWidthPercent* TEXTURE_PITCH as f64) as i32;

                let gateDistance = centeredHitInfo.GetDistToHitPoint();
                
                Some(WallSlice {
                    textureHandle: LightTexture(rayCursor, litGateTexture, unlitGateTexture),
                    textureRect: Rect::new(gateTextureX, 0, 1, TEXTURE_PITCH),
                    dist: gateDistance,
                })

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
        self.position > 0.2
    }

    pub fn Update(&mut self, moveIncr: f64, timerIncr: f64, playerInsideDoor: bool) {    
        match self.status {
            DoorStatus::CLOSED => {},
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
            }
        }
    }
}

/**************************************************************** ObjectTile ****************************************************************/

const NO_PASSTHROUGH_ID_LIST: [i32; 21] = [4, 5, 6, 8, 10, 11, 13, 14, 15, 16, 19, 20, 21, 25, 38, 39, 40, 42, 48, 49, 50];

#[derive(Clone)]
pub struct ObjectTile {
    pub sprite: Sprite,
    passthrough: bool
}

impl ObjectTile {
    pub fn New(sprite: Sprite) -> Self {
        Self {
            sprite,
            passthrough: {
                if sprite.spriteType == SpriteType::OBJECT && sprite.textureHandle.textureType == TextureType::OBJECT {
                    let objectTextureID = sprite.textureHandle.ID;
                    !NO_PASSTHROUGH_ID_LIST.contains(&objectTextureID)
                } else {
                    panic!()
                }
            }
        }
    }

    pub fn PlayerTileHit(&self) -> bool {
        return !self.passthrough;
    }
}


/**************************************************************** EmptyTile ****************************************************************/

#[derive(Clone)]
pub struct EmptyTile {
    sprites: Vec<Sprite>
}

impl EmptyTile {
    pub fn New(sprites: Option<Vec<Sprite>>) -> Self {        
        if sprites.is_some() {
            Self {
                sprites: sprites.unwrap()
            }
        } else {
            Self {
                sprites: Vec::new()
            }
        }
    }

    pub fn GetSprites(&self) -> Option<&Vec<Sprite>> {
        if self.sprites.is_empty() {
            None
        } else {
            Some(&self.sprites)
        }
    }

    pub fn PlayerTileHit() -> bool {
        return false;
    }
}
