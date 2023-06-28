
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
    pub fn LoadFromCSV(csvPath: &str) -> Self {
        let tileTextureIDs = ParseCSV(csvPath).unwrap();
        let width = tileTextureIDs.ncols() as i32;
        let height = tileTextureIDs.nrows() as i32;

        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::NONE; height as usize]; width as usize];

        for column in 0..width {
            for row in 0..height {
                let tileTextureID = *tileTextureIDs.get((row as usize, column as usize)).unwrap();
                
                tiles[column as usize][row as usize] = match tileTextureID {
                    0 => Tile::EMPTY(EmptyTile::New()),
                    99 => Tile::DOOR(Door::New()),
                    _ => Tile::WALL(Wall::New(tileTextureID, tileTextureID+1))
                }
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn GetTile(&self, tileCoord: iPoint2) -> &Tile {
        &self.tiles[tileCoord.x() as usize][tileCoord.y() as usize]
    }

    pub fn WithinMap(&self, tileCoord: iPoint2) -> bool {
        (tileCoord.x() >= 0 && tileCoord.x() < self.width) && (tileCoord.y() >= 0 && tileCoord.y() < self.height)
    }
}