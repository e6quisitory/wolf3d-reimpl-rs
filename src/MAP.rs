
/*********************************** MAP ***********************************/

use ndarray::Array2;
use crate::UTILS::VEC2D::iPoint2;
use crate::UTILS::CSV::ParseCSV;

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Array2<i32>
}

impl Map {
    pub fn LoadFromCSV(csvPath: &str) -> Self {
        let tiles = ParseCSV(csvPath).unwrap();
        let width = tiles.nrows() as i32;
        let height = tiles.ncols() as i32;

        Self {
            tiles,
            width,
            height
        }
    }

    pub fn GetTile(&self, tileCoord: iPoint2) -> i32 {
        *self.tiles.get((tileCoord.x() as usize, tileCoord.y() as usize)).unwrap()
    }

    pub fn WithinMap(&self, tileCoord: iPoint2) -> bool {
        (tileCoord.x() >= 0 && tileCoord.x() <= self.width) && (tileCoord.y() >= 0 && tileCoord.y() <= self.height)
    }
}