use crate::multimedia::TextureType;
use crate::tiles::{Wall, EmptyTile, Door, Tile, Sprite, TextureHandle, ObjectTile};
use crate::utils::vec2d::{iPoint2, Point2, Vec2, RandomUnitVec};
use crate::utils::csv::ParseCSV;

pub enum EnemyType {
    GUARD,
    OFFICER,
    SS
}

pub struct Enemy {
    pub enemyType: EnemyType,
    pub location: Point2,
    pub viewDir: Vec2,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Vec<Tile>>,
    doorTileCoords: Vec<iPoint2>,
}

impl Map {
    pub fn LoadFromCSV(csvPath: &str) -> (Self, Vec<Enemy>) {
        let tileTextureIDs = ParseCSV(csvPath).unwrap();
        let width = tileTextureIDs.ncols() as i32;
        let height = tileTextureIDs.nrows() as i32;
        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::NONE; height as usize]; width as usize];
        
        let mut doorTileCoords: Vec<iPoint2> = Vec::new();

        let mut enemies: Vec<Enemy> = Vec::new();

        for column in 0..width {
            for row in 0..height {
                let tileCode = tileTextureIDs.get((row as usize, column as usize)).unwrap();
                let tileTypeCode = tileCode.0.clone();
                let tileTextureID = tileCode.1;

                let spriteLocation = Point2::New(column as f64 + 0.5, row as f64 + 0.5);

                tiles[column as usize][row as usize] = match tileTypeCode.as_str() {
                    "" => Tile::EMPTY(EmptyTile::New(None)),
                    "W" => Tile::WALL(Wall::New(tileTextureID.unwrap(), tileTextureID.unwrap()+1)),
                    "D" => {
                        doorTileCoords.push(iPoint2::New(column, row));
                        Tile::DOOR(Door::New())
                    },
                    "GU" => {
                        let guard = Sprite {
                            textureHandle: TextureHandle::New(TextureType::GUARD, 1),
                            location: spriteLocation
                        };

                        let mut sprites: Vec<Sprite> = Vec::new();
                            sprites.push(guard);

                        enemies.push(
                            Enemy {
                                enemyType: EnemyType::GUARD,
                                location: spriteLocation,
                                viewDir: RandomUnitVec()
                            }
                        );

                        Tile::EMPTY(EmptyTile::New(Some(sprites)))
                    },
                    "OF" => {
                        let officer = Sprite {
                            textureHandle: TextureHandle::New(TextureType::OFFICER, 1),
                            location: spriteLocation
                        };

                        enemies.push(
                            Enemy {
                                enemyType: EnemyType::OFFICER,
                                location: spriteLocation,
                                viewDir: RandomUnitVec()
                            }
                        );

                        let mut sprites: Vec<Sprite> = Vec::new();
                            sprites.push(officer);

                        Tile::EMPTY(EmptyTile::New(Some(sprites)))
                    },
                    "SS" => {
                        let SS = Sprite {
                            textureHandle: TextureHandle::New(TextureType::SS, 1),
                            location: spriteLocation
                        };

                        enemies.push(
                            Enemy {
                                enemyType: EnemyType::SS,
                                location: spriteLocation,
                                viewDir: RandomUnitVec()
                            }
                        );

                        let mut sprites: Vec<Sprite> = Vec::new();
                            sprites.push(SS);

                        Tile::EMPTY(EmptyTile::New(Some(sprites)))
                    },
                    "O" => {
                        let object = Sprite {
                            textureHandle: TextureHandle::New(TextureType::OBJECT, tileTextureID.unwrap()),
                            location: spriteLocation
                        };

                        Tile::OBJECT(ObjectTile::New(object))
                    },
                    _ => panic!()
                };
            }
        }

        (
            Self {
                tiles,
                width,
                height,
                doorTileCoords,
            },
            enemies
        )

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
        for doorIndex in 0..self.doorTileCoords.len() {
            let doorCoord = self.doorTileCoords[doorIndex];
            if let Tile::DOOR(door) = self.GetMutTile(doorCoord) {
                door.Update(moveIncr, timerIncr, iPoint2::from(playerLoc) == doorCoord);
            }
        }
    }
}