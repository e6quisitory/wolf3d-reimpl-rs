
/*********************************** RAY ***********************************/

use super::vec2d::{iVec2, Point2, Vec2};
use super::conventions::*;
use super::misc_math::IsInteger;

/*
=================================================
    Ray struct definition
=================================================
*/

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub origin: Point2,
    pub direction : Vec2,

    pub xDir: xDir_t,
    pub yDir: yDir_t,
    pub xDirVec: iVec2,     // <1,0> if xDir = 1, <-1,0> if xDir = -1
    pub yDirVec: iVec2,     // <0,1> if yDir = 1, <0,-1> if yDir = -1

    pub xStep: f64,         // Amount x changes for change of 1 unit in y
    pub yStep: f64,         // Amount y changes for change of 1 unit in x

    dxConst: f64,           // Change in length along ray upon change in x of 1 unit
    dyConst: f64,           // Change in length along ray upon change in y of 1 unit
}

/*
=================================================
    Ray methods
=================================================
*/

impl Ray {

    /* Constructor */

    pub fn New(o: Point2, d: Vec2) -> Self {
        let mut r = Ray {
            origin:     o,
            direction:  d.UnitVector(),
            dxConst:    (1.0+(d.y()/d.x()).powi(2)).sqrt(),
            dyConst:    (1.0+(d.x()/d.y()).powi(2)).sqrt(),
            xDir:       if d.x() > 0.0 { xDir_t::EAST } else { xDir_t::WEST },
            yDir:       if d.y() > 0.0 { yDir_t::NORTH } else { yDir_t::SOUTH },
            ..Default::default()
        };

        r.xDirVec = iVec2::New(r.xDir as i32, 0);
        r.yDirVec = iVec2::New(0, r.yDir as i32);

        r.yStep = (r.dxConst.powi(2) - 1.0).sqrt();
        r.xStep = (r.dyConst.powi(2) - 1.0).sqrt();

        return r;
    }

    /* Private methods */

    fn yAt(&self, x: f64) -> f64 {
        self.origin.y() + ((x-self.origin.x())/self.direction.x())*self.direction.y()
    }

    fn xAt(&self, y: f64) -> f64 {
        self.origin.x() + ((y-self.origin.y())/self.direction.y())*self.direction.x()
    }

    fn nextX(&self, currPt: Point2) -> i32 {
        if !IsInteger(currPt.x()) {
            match self.xDir {
                xDir_t::EAST => currPt.x().ceil() as i32,
                xDir_t::WEST => currPt.x().floor() as i32,
                xDir_t::NONE => -1
            }
        } else {
            currPt.x() as i32 + self.xDir as i32
        }
    }

    fn nextY(&self, currPt: Point2) -> i32 {
        if !IsInteger(currPt.y()) {
            match self.yDir {
                yDir_t::NORTH => currPt.y().ceil() as i32,
                yDir_t::SOUTH => currPt.y().floor() as i32,
                yDir_t::NONE => -1
            }
        } else {
            currPt.y() as i32 + self.yDir as i32
        }
    }

    fn RayDist_dx(&self, dx: f64) -> f64 {
        dx.abs()*self.dxConst
    }

    fn RayDist_dy(&self, dy: f64) -> f64 {
        dy.abs()*self.dyConst
    }

    /* Public methods */

    pub fn NextXIntrscPoint(&self, currPt: Point2) -> Point2 {
        let nextXVal = self.nextX(currPt) as f64;
        Point2::New(nextXVal, self.yAt(nextXVal))
    }

    pub fn NextYIntrscPoint(&self, currPt: Point2) -> Point2 {
        let nextYVal = self. nextY(currPt) as f64;
        Point2::New(self.xAt(nextYVal), nextYVal)
    }

    pub fn DistToPoint(&self, pt: Point2) -> f64 {
        self.RayDist_dx(pt.x() - self.origin.x())
    }
}
