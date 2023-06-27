/*********************************** TILES ***********************************/

use std::rc::Rc;

use sdl2::{render::Texture, rect::Rect};
use crate::{UTILS::{VEC2D::{Point2, Vec2}, DDA::RayCursor, CONVENTIONS::TEXTURE_PITCH}, multimedia::LightTexture};

/**************** Types ****************/

pub struct TextureSlice {
    pub texture: Rc<Texture>,
    pub slice: Rect
}

pub struct TextureSliceDistPair {
    pub textureSlice: TextureSlice,
    pub dist: f64
}

pub struct SpriteRenderData {
    pub location: Point2,
    pub texture: Rc<Texture>
}

pub struct TexturePair {
    pub lit: Rc<Texture>,
    pub unlit: Rc<Texture>
}

pub enum rayTileHitReturn_t<'a> {
    WALL(TextureSliceDistPair),
    SPRITE(SpriteRenderData),
    WALL_AND_SPRITES((TextureSliceDistPair, &'a mut Vec<SpriteRenderData>)),
    SPRITES(&'a mut Vec<SpriteRenderData>)
}

#[derive(Clone, Copy, PartialEq)]
pub enum tileType_t {
    WALL,
    DOOR,
    EMPTY,
    OBJECT,
    COLLECTIBLE
}

/**************** Hittable Trait ****************/

pub trait Hittable {
    fn GetTileType(&self) -> tileType_t;
    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t>;
    fn PlayerTileHit(&self) -> bool;
    fn Update(&mut self, incr: f64);
}

/**************** Wall ****************/

pub struct Wall {
    pub texturePair: TexturePair
}

impl Hittable for Wall {
    fn GetTileType(&self) -> tileType_t {
        return tileType_t::WALL;
    }

    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        let widthPercent = rayCursor.GetWidthPercent();
        let textureX = (widthPercent * TEXTURE_PITCH as f64) as i32;
        let texture = LightTexture(&self.texturePair, rayCursor.GetWallType());

        let textureSlice = TextureSlice {
            texture: texture,
            slice: Rect::new(textureX, 0, 1, TEXTURE_PITCH)
        };

        let textureSliceDistPair = TextureSliceDistPair {
            textureSlice,
            dist: rayCursor.GetDistToHitPoint()
        };

        return Some(rayTileHitReturn_t::WALL(textureSliceDistPair));
    }

    fn PlayerTileHit(&self) -> bool {
        return true;
    }

    fn Update(&mut self, incr: f64) {}
}

/**************** Door ****************/

pub enum doorStatus_t {
    OPEN,
    CLOSED,
    OPENING,
    CLOSING
}

pub enum doorPosition_t {
    OPEN = 0,
    CLOSED = 1
}

pub enum doorTimerVal_t {
    NO_TIME_LEFT   = 0,
    FULL_TIME_LEFT = 1
}

pub struct Door {
    pub position: f64,
    pub status: doorStatus_t,
    pub timerVal: f64,
    pub gateTexturePair: TexturePair,
    pub enemiesWithin: Vec<Enemy>,
    pub spriteRenderDataList: Vec<SpriteRenderData>
}

impl Door {
    pub fn New(gateTexturePair: TexturePair) -> Self {
        Self {
            position: doorPosition_t::CLOSED as i32 as f64,
            status: doorStatus_t::CLOSED,
            timerVal: doorTimerVal_t::FULL_TIME_LEFT as i32 as f64,
            gateTexturePair,
            enemiesWithin: Vec::new(),
            spriteRenderDataList: Vec::new()
        }
    }
}

impl Hittable for Door {    
    fn GetTileType(&self) -> tileType_t {
        return tileType_t::DOOR;
    }

    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        // Center hit point
        let mut centeredHitInfo = rayCursor.GetNextCenterHit();

        // First check if incoming ray actually intersects with middle of tile (the gate)
        if centeredHitInfo.hitTile == rayCursor.hitTile {

            let centerWidthPercent = centeredHitInfo.GetWidthPercent();

            // Ray does intersect gate, but now check if the gate *blocks* the ray
            if centerWidthPercent < self.position {

                // If ray is blocked by gate, then output the proper gate texture and rect
                let gateTexture = LightTexture(&self.gateTexturePair, rayCursor.GetWallType());
                let gateWidthPercent = self.position - centerWidthPercent;

                let gateTextureX = (gateWidthPercent* TEXTURE_PITCH as f64) as i32;
                let gateTextureSlice = TextureSlice {
                    texture: gateTexture,
                    slice: Rect::new(gateTextureX, 0, 1, TEXTURE_PITCH)
                };
                let gateDistance = centeredHitInfo.GetDistToHitPoint();
                let gateTextureSliceDistPair = TextureSliceDistPair {
                    textureSlice: gateTextureSlice,
                    dist: gateDistance,
                };

                return Some(rayTileHitReturn_t::WALL(gateTextureSliceDistPair));
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

    fn PlayerTileHit(&self) -> bool {
        return true;
    }

    fn Update(&mut self, incr: f64) {        
        if self.position > 0.0 {
            self.position -= incr/10.0;
        } 
    }
}

/**************** EmptyTile ****************/

pub struct EmptyTile {
    pub enemiesWithin: Vec<Enemy>,
    pub spriteRenderDataList: Vec<SpriteRenderData>,
}

impl Hittable for EmptyTile {
    fn GetTileType(&self) -> tileType_t {
        return tileType_t::EMPTY;
    }

    fn RayTileHit(&self, _rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        return None;
    }

    fn PlayerTileHit(&self) -> bool {
        return false;
    }

    fn Update(&mut self, incr: f64) {}
}

/**************** Object ****************/

pub struct Object {
    pub texture: Rc<Texture>
}

/**************** Collectible ****************/

pub struct Collectible {
    pub texture: Rc<Texture>,
    pub collected: bool
}

/**************** Enemy ****************/

pub struct Enemy {
    pub position: Point2,
    pub viewDir: Vec2,
    pub texture: Rc<Texture>
}