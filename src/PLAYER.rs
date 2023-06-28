
/*********************************** INPUTS_BUFFER ***********************************/

use crate::inputs_buffer::{InputsBuffer, lookCommand_t, moveCommand_t};
use crate::map::Map;
use crate::tiles::{Tile, DoorStatus};
use crate::utils::conventions::PI;
use crate::utils::dda::RayCursor;
use crate::utils::ray::Ray;
use crate::utils::vec2d::Vec2;
use super::utils::vec2d::Point2;
use crate::inputs_buffer::doorCommand_t;

pub struct Player {
    pub position: Point2,
    pub viewDir: Vec2,
    pub east: Vec2,
    pub west: Vec2,
}

impl Player {
    pub fn New() -> Self {
        let position = Point2::New(4.127, 5.033);
        let viewDir = Point2::New(-0.019038625821465295, 0.7068504302374231).UnitVector();
        let east = viewDir.Rotate(-PI/2.0);
        let west = viewDir.Rotate(PI/2.0);

        Player {
            position,
            viewDir,
            east,
            west
        }
    }

    pub fn Update(&mut self, inputsBuffer: &InputsBuffer, map: &mut Map) {
        self.east = self.viewDir.Rotate(-PI/2.0);
        self.west = self.viewDir.Rotate(PI/2.0);

        const swivelIncr: f64 = 0.04;
        const moveIncr: f64 = 0.1;

        let mut proposedLoc: Point2 = self.position;

        match inputsBuffer.moveCommand {
            moveCommand_t::NORTH => { proposedLoc = self.position + self.viewDir*moveIncr; }
            moveCommand_t::SOUTH => { proposedLoc = self.position - self.viewDir*moveIncr; }
            moveCommand_t::EAST => { proposedLoc = self.position + self.east*moveIncr; }
            moveCommand_t::WEST => { proposedLoc = self.position + self.west*moveIncr; }
            moveCommand_t::NORTH_EAST => { proposedLoc = self.position + self.viewDir*moveIncr*0.7071067 + self.east*moveIncr*0.7071067; }
            moveCommand_t::NORTH_WEST => { proposedLoc = self.position + self.viewDir*moveIncr*0.7071067 + self.west*moveIncr*0.7071067; }
            moveCommand_t::NONE => {}
        }
        self.MoveIfValid(proposedLoc, map);

        match inputsBuffer.lookCommand {
            lookCommand_t::RIGHT => { self.viewDir = self.viewDir.Rotate(-swivelIncr); }
            lookCommand_t::LEFT => { self.viewDir = self.viewDir.Rotate(swivelIncr); }
            lookCommand_t::NONE => {}
        }

        match inputsBuffer.doorCommand {
            doorCommand_t::OPEN => {
                let mut rayCursor = RayCursor::New(Ray::New(self.position, self.viewDir), self.position);
                while map.WithinMap(rayCursor.hitTile) {
                    rayCursor.GoToNextHit();
                    if rayCursor.GetDistToHitPoint() > 4.0 {
                        break;
                    } else {
                        match map.GetMutTile(rayCursor.hitTile) {
                            Tile::EMPTY(_) => {
                                continue;
                            },
                            Tile::DOOR(hitDoor) => {
                                if hitDoor.status == DoorStatus::CLOSED || hitDoor.status == DoorStatus::CLOSING {
                                    (*hitDoor).status = DoorStatus::OPENING;
                                    break;
                                }
                            },
                            Tile::NONE => panic!(),
                            _ => {
                                break;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn MoveIfValid(&mut self, proposedLocation: Point2, map: &Map) {
        match map.GetTile(proposedLocation.into()) {
            crate::tiles::Tile::EMPTY(_) => {
                self.position = proposedLocation;
            },
            crate::tiles::Tile::DOOR(door) => {
                if !door.PlayerTileHit() {
                    self.position = proposedLocation;
                }
            },
            crate::tiles::Tile::NONE => panic!(),
            _ => {}
        }
    }
}