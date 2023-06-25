
/*********************************** MAP ***********************************/

use std::rc::Rc;

use crate::MULTIMEDIA::Assets;
use crate::TILES::{Hittable, Wall, TexturePair, EmptyTile};
use crate::UTILS::VEC2D::iPoint2;
use crate::UTILS::CSV::ParseCSV;

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Vec<Option<Box<dyn Hittable>>>>
}

impl Map {
    pub fn LoadFromCSV(csvPath: &str, assets: &Assets) -> Self {
        let tileTextureIDs = ParseCSV(csvPath).unwrap();
        let width = tileTextureIDs.ncols() as i32;
        let height = tileTextureIDs.nrows() as i32;

        let mut tiles: Vec<Vec<Option<Box<dyn Hittable>>>> = Vec::new();

        for row in 0..=height-1 {
            tiles.push(Vec::new());
            for _ in 0..=width-1 {
                tiles[row as usize].push(None);
            }
        }

        for row in 0..=height-1 {
            for column in 0..=width-1 {
                tiles[row as usize][column as usize] = {
                    let tileTextureID = *tileTextureIDs.get((row as usize, column as usize)).unwrap();
                    match tileTextureID {
                        0 => Some(Box::new(EmptyTile {
                            enemiesWithin: Vec::new(),
                            spriteRenderDataList: Vec::new(),
                        })),
                        _ => Some(Box::new(Wall {
                            texturePair: TexturePair {
                                lit: Rc::clone(&assets.wallTextures[(tileTextureID-1) as usize]),
                                unlit: Rc::clone(&assets.wallTextures[tileTextureID as usize]),
                            }
                        }))
                    }
                }
            }
        }

        Self {
            tiles,
            width,
            height
        }
    }

    pub fn GetTile(&self, tileCoord: iPoint2) -> Option<&Box<dyn Hittable>> {
        self.tiles[tileCoord.y() as usize][tileCoord.x() as usize].as_ref()
    }

    pub fn WithinMap(&self, tileCoord: iPoint2) -> bool {
        (tileCoord.x() >= 0 && tileCoord.x() < self.width) && (tileCoord.y() >= 0 && tileCoord.y() < self.height)
    }
}