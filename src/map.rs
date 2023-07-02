use crate::multimedia::TextureType;
use crate::tiles::{Wall, EmptyTile, Door, Tile, Sprite, TextureHandle};
use crate::utils::vec2d::{iPoint2, Point2};
use crate::utils::csv::ParseCSV;

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Vec<Tile>>,

    // Doors related
    doorCoords: Vec<iPoint2>,
    numDoors: usize
}

impl Map {
    pub fn LoadFromCSV(csvPath: &str) -> Self {
        let tileTextureIDs = ParseCSV(csvPath).unwrap();
        let width = tileTextureIDs.ncols() as i32;
        let height = tileTextureIDs.nrows() as i32;
        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::NONE; height as usize]; width as usize];
        
        let mut doorCoords: Vec<iPoint2> = Vec::new();
        let mut numDoors: usize = 0;

        for column in 0..width {
            for row in 0..height {
                let tileTextureID = *tileTextureIDs.get((row as usize, column as usize)).unwrap();
                
                tiles[column as usize][row as usize] = match tileTextureID {
                    0 => Tile::EMPTY(EmptyTile::New(None)),
                    69 => {
                        let dead_enemy = Sprite {
                            textureHandle: TextureHandle::New(TextureType::GUARD, 45),
                            location: Point2::New(column as f64 + 0.5, row as f64 + 0.5)
                        };

                        let ammo = Sprite {
                            textureHandle: TextureHandle::New(TextureType::OBJECT, 29),
                            location: Point2::New(column as f64 + 0.15, row as f64 + 0.15)
                        };

                        let mut sprites: Vec<Sprite> = Vec::new();
                            sprites.push(dead_enemy);
                            sprites.push(ammo);

                        Tile::EMPTY(EmptyTile::New(Some(sprites)))
                    },
                    420 => {
                        let plant = Sprite {
                            textureHandle: TextureHandle::New(TextureType::OBJECT, 11),
                            location: Point2::New(column as f64 + 0.15, row as f64 + 0.15)
                        };

                        let mut sprites: Vec<Sprite> = Vec::new();
                            sprites.push(plant);

                        Tile::EMPTY(EmptyTile::New(Some(sprites)))
                    },
                    42 => {
                        let SS = Sprite {
                            textureHandle: TextureHandle::New(TextureType::SS, 51),
                            location: Point2::New(column as f64 + 0.15, row as f64 + 0.15)
                        };

                        let mut sprites: Vec<Sprite> = Vec::new();
                            sprites.push(SS);

                        Tile::EMPTY(EmptyTile::New(Some(sprites)))
                    },
                    99 => {
                        doorCoords.push(iPoint2::New(column, row));
                        numDoors += 1;
                        
                        Tile::DOOR(Door::New())
                    },
                    _ => Tile::WALL(Wall::New(tileTextureID, tileTextureID+1))
                };
            }
        }

        Self {
            tiles,
            width,
            height,
            doorCoords,
            numDoors
        }

    }  

    pub fn GetTile(&self, tileCoord: iPoint2) -> &Tile {
        &self.tiles[tileCoord.x() as usize][tileCoord.y() as usize]
    }

    pub fn GetMutTile(&mut self, tileCoord: iPoint2) -> &mut Tile {
        &mut self.tiles[tileCoord.x() as usize][tileCoord.y() as usize]
    }

    pub fn WithinMap(&self, tileCoord: iPoint2) -> bool {
        (tileCoord.x() > 0 && tileCoord.x() < self.width-1) && (tileCoord.y() > 0 && tileCoord.y() < self.height-1)
    }

    pub fn UpdateDoors(&mut self, moveIncr: f64, timerIncr: f64, playerLoc: Point2) {
        for doorIndex in 0..self.numDoors {
            let doorCoord = self.doorCoords[doorIndex];
            if let Tile::DOOR(door) = self.GetMutTile(doorCoord) {
                door.Update(moveIncr, timerIncr, iPoint2::from(playerLoc) == doorCoord);
            }
        }
    }
}