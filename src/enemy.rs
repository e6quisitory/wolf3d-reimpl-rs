use crate::multimedia::TextureType;
use crate::tiles::{Sprite, TextureHandle};
use crate::utils::vec2d::{Dot, iPoint2, Point2, RandomUnitVec, Vec2};
use std::f64::consts::PI;
use crate::animation::{AnimationClip, AnimationMagazine, AnimationReel};
use crate::map::Map;
use crate::player::Player;

#[derive(PartialEq)]
pub enum EnemyType {
    GUARD,
    OFFICER,
    SS
}

#[derive(PartialEq)]
enum EnemyState {
    IDLE,
    DAMAGE,
    DEAD
}

pub struct EnemyInputsBuffer {
    pub damage: bool
}

impl EnemyInputsBuffer {
    pub fn New() -> Self {
        Self {
            damage: false
        }
    }
}

pub struct Enemy {
    pub enemyType: EnemyType,
    pub location: Point2,
    pub tile: iPoint2,
    pub viewDir: Vec2,
    pub AM_enemySprites: AnimationMagazine,
    currState: EnemyState,
    pub inputsBuffer: EnemyInputsBuffer,
    pub health: i32
}

fn GenerateEnemyAnimationMagazine(textureType: TextureType) -> AnimationMagazine {
    let mut AM = AnimationMagazine::New(Vec::new(), 0);
    // Clips 0 - 7
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

    // Clip 8 - damage
    AM.clips.push(
        AnimationClip::REEL(
            AnimationReel::New(
                vec![
                    TextureHandle::New(textureType, 41)
                ],
                0.3,
                0.02,
                Some(0)
            )
        )
    );

    // Clip 9 - death animation
    AM.clips.push(
        AnimationClip::REEL(
            AnimationReel::New(
                vec![
                    TextureHandle::New(textureType, 42),
                    TextureHandle::New(textureType, 43),
                    TextureHandle::New(textureType, 44),
                    TextureHandle::New(textureType, 45)
                ],
                0.3,
                0.045,
                Some(10)
            )
        )
    );

    // Clip 10 - dead body (static)
    AM.clips.push(
        AnimationClip::STATIC (
            TextureHandle::New(textureType, 45)
        )
    );

    return AM;
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
            AM_enemySprites,
            currState: EnemyState::IDLE,
            inputsBuffer: EnemyInputsBuffer::New(),
            health: 150
        }
    }

    pub fn Update(&mut self, map: &Map, player: &Player) {
        match self.currState {
            EnemyState::IDLE => {
                if self.inputsBuffer.damage == true {
                    self.inputsBuffer.damage = false;
                    self.currState = EnemyState::DAMAGE;
                } else if self.AM_enemySprites.currClipIndex < 8 {
                    let proposedLocation = self.location + self.viewDir*0.01;
                    let proposedTileCoord = iPoint2::from(proposedLocation);
                    if map.ValidEnemyLocation(proposedLocation, player.location) {
                        self.location = proposedLocation;
                        self.tile = proposedTileCoord;
                    } else {
                        self.viewDir = RandomUnitVec();
                    }
                }
            },
            EnemyState::DAMAGE => {
                self.health -= 20;
                if self.health > 0 {
                    self.AM_enemySprites.currClipIndex = 8;
                    self.currState = EnemyState::IDLE;
                } else {
                    self.AM_enemySprites.currClipIndex = 9;
                    self.currState = EnemyState::DEAD;
                }
            },
            EnemyState::DEAD => {
                // stay dead lol
            }
        }

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

        let angleCorrectClipIndex = {
            if (angle >= 15.0*PI/8.0 && angle <= 2.0*PI) || (angle >= 0.0 && angle < PI/8.0) {
                2
            } else if angle >= PI/8.0 && angle < 3.0*PI/8.0 {
                3
            } else if angle >= 3.0*PI/8.0 && angle < 5.0*PI/8.0 {
                4
            } else if angle >= 5.0*PI/8.0 && angle < 7.0*PI/8.0 {
                5
            } else if angle >= 7.0*PI/8.0 && angle < 9.0*PI/8.0 {
                6
            } else if angle >= 9.0*PI/8.0 && angle < 11.0*PI/8.0 {
                7
            } else if angle >= 11.0*PI/8.0 && angle < 13.0*PI/8.0 {
                0
            } else {
                1
            }
        };

        let currClipIndex = self.AM_enemySprites.currClipIndex;
        if currClipIndex < 8 && angleCorrectClipIndex != currClipIndex {
            self.AM_enemySprites.SwitchClipIndexWithTimeCopy(angleCorrectClipIndex as usize);
        }

        let textureHandle = self.AM_enemySprites.GetCurrTexture();

        Sprite {
            textureHandle,
            location: self.location
        }
    }
}
