use crate::multimedia::TextureType;
use crate::tiles::{Sprite, TextureHandle};
use crate::utils::vec2d::{Dot, iPoint2, Point2, Vec2};
use std::f64::consts::PI;
use crate::animation::{AnimationClip, AnimationMagazine, AnimationReel};

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
    pub AM_enemySprites: AnimationMagazine
}

fn GenerateEnemyAnimationMagazine(textureType: TextureType) -> AnimationMagazine {
    let mut AM = AnimationMagazine::New(Vec::new(), 0);
    for i in 0..8 {
        AM.clips.push(
            AnimationClip::REEL(
                AnimationReel::New(
                    vec![
                        //TextureHandle::New(textureType, 1+i),
                        TextureHandle::New(textureType, 9+i),
                        TextureHandle::New(textureType, 17+i),
                        TextureHandle::New(textureType, 25+i),
                        TextureHandle::New(textureType, 33+i)
                    ],
                    0.3,
                    0.02,
                    None
                )
            )
        );
    }
    AM
}

impl Enemy {
    pub fn New(enemyType: EnemyType, location: Point2, tile: iPoint2, viewDir: Vec2) -> Self {
        let AM_enemySprites: AnimationMagazine =  match enemyType {
            EnemyType::GUARD => {
                GenerateEnemyAnimationMagazine(TextureType::GUARD)
            },
            EnemyType::OFFICER => {
                GenerateEnemyAnimationMagazine(TextureType::OFFICER)
            },
            EnemyType::SS => {
                GenerateEnemyAnimationMagazine(TextureType::SS)
            },
        };

        Self {
            enemyType,
            location,
            tile,
            viewDir,
            AM_enemySprites
        }
    }

    pub fn Update(&mut self) {
        self.AM_enemySprites.Update();
    }

    pub fn CalculateSprite(&mut self, playerViewDir: Vec2) -> Sprite {
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

        let currClipIndex = {
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
        };

        let textureHandle = self.AM_enemySprites.GetCurrEnemyTexture(currClipIndex-1);

        Sprite {
            textureHandle,
            location: self.location
        }
    }
}