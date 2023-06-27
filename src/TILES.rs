/*********************************** TILES ***********************************/

use std::rc::Rc;

use sdl2::{render::Texture};
use crate::{UTILS::{VEC2D::{Point2, Vec2}, DDA::RayCursor, CONVENTIONS::TEXTURE_PITCH}, MULTIMEDIA::LightTexture};

/**************** Types ****************/

pub struct TextureSlice {
    pub texture: Rc<Texture>,
    pub sliceX: i32
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

/**************** Hittable Trait ****************/

pub trait Hittable {
    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t>;
    fn PlayerTileHit(&self) -> bool;
    fn IsDoor(&self) -> bool;
}

/**************** Wall ****************/

pub struct Wall {
    pub texturePair: TexturePair
}

impl Hittable for Wall {
    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        let widthPercent = rayCursor.GetWidthPercent();
        let textureX = (widthPercent * TEXTURE_PITCH as f64) as i32;
        let texture = LightTexture(&self.texturePair, rayCursor.GetWallType());

        let textureSlice = TextureSlice {
            texture: texture,
            sliceX: textureX
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

    fn IsDoor(&self) -> bool {
        return false;
    }
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
    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        // Center hit point
        let mut centeredHitInfo = rayCursor.GetNextCenterHit();

        // First check if incoming ray actually intersects with middle of tile (the gate)
        if centeredHitInfo.hitTile == rayCursor.hitTile {

            // Ray does intersect gate, but now check if the gate *blocks* the ray
            if centeredHitInfo.GetWidthPercent() < self.position {

                // If ray is blocked by gate, then output the proper gate texture and rect
                let gateTexture = LightTexture(&self.gateTexturePair, rayCursor.GetWallType());

                let gateTextureX = (centeredHitInfo.GetWidthPercent() * TEXTURE_PITCH as f64) as i32;
                let gateTextureSlice = TextureSlice {
                    texture: gateTexture,
                    sliceX: gateTextureX,
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

    fn IsDoor(&self) -> bool {
        return true;
    }
}

/**************** EmptyTile ****************/

pub struct EmptyTile {
    pub enemiesWithin: Vec<Enemy>,
    pub spriteRenderDataList: Vec<SpriteRenderData>,
}

impl Hittable for EmptyTile {
    fn RayTileHit(&self, _rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        return None;
    }

    fn PlayerTileHit(&self) -> bool {
        return false;
    }

    fn IsDoor(&self) -> bool {
        return false;
    }
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