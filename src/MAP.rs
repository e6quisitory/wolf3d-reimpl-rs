
/*********************************** MAP ***********************************/

use crate::tiles::{Wall, EmptyTile, Door, Tile};
use crate::utils::vec2d::iPoint2;
use crate::utils::csv::ParseCSV;

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn LoadFromCSV(csvPath: &str) -> (Self, Vec<iPoint2>) {
        let tileTextureIDs = ParseCSV(csvPath).unwrap();
        let width = tileTextureIDs.ncols() as i32;
        let height = tileTextureIDs.nrows() as i32;

        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::NONE; height as usize]; width as usize];
        let mut doors: Vec<iPoint2> = Vec::new();

        for column in 0..width {
            for row in 0..height {
                let tileTextureID = *tileTextureIDs.get((row as usize, column as usize)).unwrap();
                
                tiles[column as usize][row as usize] = match tileTextureID {
                    0 => Tile::EMPTY(EmptyTile::New()),
                    99 => Tile::DOOR(Door::New()),
                    _ => Tile::WALL(Wall::New(tileTextureID, tileTextureID+1))
                };

                if tileTextureID == 99 {
                    doors.push(iPoint2::New(column, row));
                }
            }
        }

        (
            Self {
                tiles,
                width,
                height,
            },
            doors
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
}