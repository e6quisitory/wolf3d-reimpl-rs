use crate::multimedia::TextureType;
use crate::tiles::{Sprite, TextureHandle};
use crate::utils::vec2d::{Dot, iPoint2, Point2, Vec2};
use std::f64::consts::PI;

pub enum EnemyType {
    GUARD,
    OFFICER,
    SS
}

pub struct Enemy {
    pub enemyType: EnemyType,
    pub location: Point2,
    pub tile: iPoint2,
    pub viewDir: Vec2,

    pub walkTimer: f64,
    pub walkSpriteNum: i32
}

impl Enemy {
    pub fn CalculateSprite(&self, playerViewDir: Vec2) -> Sprite {
        let textureType = match self.enemyType {
            EnemyType::GUARD => TextureType::GUARD,
            EnemyType::OFFICER => TextureType::OFFICER,
            EnemyType::SS => TextureType::SS
        };

        let enemyViewDir = self.viewDir;
        let enemyEastDir = self.viewDir.Rotate(-PI/2.0);

        let playerViewDotEnemyView = Dot(playerViewDir, enemyViewDir);
        let playerViewDotEnemyEast = Dot(playerViewDir, enemyEastDir);

        let angle = {
            if playerViewDotEnemyView >= 0.0 {
                playerViewDotEnemyEast.acos()
            } else {
                2.0*PI - playerViewDotEnemyEast.acos()
            }
        };

        let textureID = {
            self.CalculateWalkingSpriteID(
                {
                    if (angle >= 15.0*PI/8.0 && angle <= 2.0*PI) || (angle >= 0.0 && angle < PI/8.0) {
                        3
                    } else if angle >= PI/8.0 && angle < 3.0*PI/8.0 {
                        4
                    } else if angle >= 3.0*PI/8.0 && angle < 5.0*PI/8.0 {
                        5
                    } else if angle >= 5.0*PI/8.0 && angle < 7.0*PI/8.0 {
                        6
                    } else if angle >= 7.0*PI/8.0 && angle < 9.0*PI/8.0 {
                        7
                    } else if angle >= 9.0*PI/8.0 && angle < 11.0*PI/8.0 {
                        8
                    } else if angle >= 11.0*PI/8.0 && angle < 13.0*PI/8.0 {
                        1
                    } else {
                        2
                    }
                }
            )
        };

        let textureHandle = TextureHandle {
            textureType,
            ID: textureID
        };

        Sprite {
            textureHandle,
            location: self.location
        }
    }

    fn CalculateWalkingSpriteID(&self, columnNum: i32) -> i32 {
        columnNum + 8*self.walkSpriteNum
    }
}