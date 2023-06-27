
/*********************************** MAP ***********************************/

use std::rc::Rc;

use crate::multimedia::Assets;
use crate::tiles::{Hittable, Wall, TexturePair, EmptyTile, Door};
use crate::UTILS::VEC2D::iPoint2;
use crate::UTILS::CSV::ParseCSV;

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Vec<Option<Box<dyn Hittable>>>>,
    
    // Doors
    pub doorCoords: Vec<iPoint2>,
    pub numDoors: i32,
}

impl Map {
    pub fn LoadFromCSV(csvPath: &str, assets: &Assets) -> Self {
        let tileTextureIDs = ParseCSV(csvPath).unwrap();
        let width = tileTextureIDs.ncols() as i32;
        let height = tileTextureIDs.nrows() as i32;

        let mut tiles: Vec<Vec<Option<Box<dyn Hittable>>>> = Vec::new();
        let mut doorCoords: Vec<iPoint2> = Vec::new();

        for row in 0..=height-1 {
            tiles.push(Vec::new());
            for _ in 0..=width-1 {
                tiles[row as usize].push(None);
            }
        }

        for row in 0..=height-1 {
            for column in 0..=width-1 {
                let tileTextureID = *tileTextureIDs.get((row as usize, column as usize)).unwrap();
                let currTile = &mut tiles[row as usize][column as usize];
                match tileTextureID {
                    0 => *currTile = Some(Box::new(EmptyTile {
                                        enemiesWithin: Vec::new(),
                                        spriteRenderDataList: Vec::new(),
                                    })),
                    99 => {
                        let currDoor = Door::New(
                            TexturePair {
                                lit: assets.GetWallTexture(99),
                                unlit: assets.GetWallTexture(100)
                            }
                        );
                        *currTile = Some(Box::new(currDoor));
                        doorCoords.push(iPoint2::New(column, row));
                    },
                    _ => *currTile = Some(Box::new(Wall {
                        texturePair: TexturePair {
                            lit: assets.GetWallTexture(tileTextureID),
                            unlit: assets.GetWallTexture(tileTextureID+1),
                        }
                    }))
                };
            }
        }

        let numDoors = doorCoords.len() as i32;

        Self {
            tiles,
            width,
            height,
            doorCoords,
            numDoors
        }
    }

    pub fn UpdateDoors(&mut self) {
        for doorCoordIndex in 0..=self.numDoors-1 {
            self.GetMutTile(self.doorCoords[doorCoordIndex as usize]).unwrap().Update(0.02);
        }
    }

    pub fn GetTile(&self, tileCoord: iPoint2) -> Option<&Box<dyn Hittable>> {
        self.tiles[tileCoord.y() as usize][tileCoord.x() as usize].as_ref()
    }

    pub fn GetMutTile(&mut self, tileCoord: iPoint2) -> Option<&mut Box<dyn Hittable>> {
        self.tiles[tileCoord.y() as usize][tileCoord.x() as usize].as_mut()
    }

    pub fn WithinMap(&self, tileCoord: iPoint2) -> bool {
        (tileCoord.x() >= 0 && tileCoord.x() < self.width) && (tileCoord.y() >= 0 && tileCoord.y() < self.height)
    }
}