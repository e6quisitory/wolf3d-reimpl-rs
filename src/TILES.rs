/*********************************** TILES ***********************************/

use std::rc::Rc;

use sdl2::{render::Texture};
use crate::{UTILS::{VEC2D::{Point2, Vec2}, DDA::{RayCursor, wallType_t}, CONVENTIONS::TEXTURE_PITCH}};

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
}

/**************** Wall ****************/

pub struct Wall {
    pub texturePair: TexturePair
}

impl Hittable for Wall {
    fn RayTileHit(&self, rayCursor: &mut RayCursor) -> Option<rayTileHitReturn_t> {
        let widthPercent = rayCursor.GetWidthPercent();
        let textureX = (widthPercent * TEXTURE_PITCH as f64) as i32;
        let texture = {
            match rayCursor.GetWallType() {
                wallType_t::HORIZONTAL => Rc::clone(&self.texturePair.unlit),
                wallType_t::VERTICAL => Rc::clone(&self.texturePair.lit),
                wallType_t::CORNER => Rc::clone(&self.texturePair.unlit),
                wallType_t::NONE => panic!(),
            }
        };

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
}

/**************** Door ****************/

pub enum doorStatus_t {
    OPEN,
    CLOSED,
    OPENING,
    CLOSING
}

pub struct Door {
    pub position: f64,
    pub status: doorStatus_t,
    pub gateTexturePair: TexturePair,
    pub sidewallTexturePair: TexturePair,
    pub enemiesWithin: Vec<Enemy>,
    pub spriteRenderDataList: Vec<SpriteRenderData>
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