
/*********************************** INPUTS_BUFFER ***********************************/

use crate::INPUTS_BUFFER::{InputsBuffer, lookCommand_t, moveCommand_t};
use crate::MAP::Map;
use crate::UTILS::CONVENTIONS::PI;
use crate::UTILS::VEC2D::Vec2;
use super::UTILS::VEC2D::Point2;

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

    pub fn Update(&mut self, inputsBuffer: &InputsBuffer, map: &Map) {
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
    }

    fn MoveIfValid(&mut self, proposedLocation: Point2, map: &Map) {
        if map.GetTile(proposedLocation.into()) == 0 {
            self.position = proposedLocation;
        }
    }
}