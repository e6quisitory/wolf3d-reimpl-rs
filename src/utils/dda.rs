use super::ray::*;
use super::vec2d::{iPoint2, Point2, Vec2};
use super::misc_math::*;

/*
=========================================================
    Relevant type definitions
=========================================================
*/

#[derive(Default, Copy, Clone, PartialEq)]
pub enum wallType_t {
    HORIZONTAL,
    VERTICAL,
    CORNER,

    #[default]
    NONE
}

/*
=========================================================
    RayCursor struct definition
=========================================================
*/

#[derive(Default, Copy, Clone)]
pub struct RayCursor {
    pub ray: Ray,
    pub hitPoint: Point2,
    pub hitTile: iPoint2,

    wallType: wallType_t,
    widthPercent: f64
}

/*
=========================================================
    RayCursor methods
=========================================================
*/

impl RayCursor {

    /* Constructors */

    pub fn New(ray: Ray, hitPoint: Point2) -> Self {
        RayCursor {
            ray,
            hitPoint,
            hitTile: iPoint2::from(hitPoint),
            widthPercent: -1.0,
            wallType: wallType_t::NONE
        }
    }

    /* Private Methods */

    fn CalculateWallHitInfo(&mut self) {
        let xDecimal = GetDecimal(self.hitPoint.x());
        let yDecimal = GetDecimal(self.hitPoint.y());
        let xIsInt = xDecimal == 0.0;
        let yIsInt = yDecimal == 0.0;
        let xIsMiddle = xDecimal == 0.5;
        let yIsMiddle = yDecimal == 0.5;

        if (xIsInt || xIsMiddle) && !yIsInt {
            self.wallType = wallType_t::VERTICAL;
            self.widthPercent = yDecimal;
        } else if (yIsInt || yIsMiddle) && !xIsInt {
            self.wallType = wallType_t::HORIZONTAL;
            self.widthPercent = xDecimal;
        } else if xIsInt && yIsInt {
            self.wallType = wallType_t::CORNER;
            self.widthPercent = 0.0;
        } else {
            self.wallType = wallType_t::NONE;
            self.widthPercent = -1.0;
        }
    }

    fn ClearWallHitInfo(&mut self) {
        self.wallType = wallType_t::NONE;
        self.widthPercent = -1.0;
    }

    /* Public methods */

    pub fn GetWallType(&mut self) -> wallType_t{
        if self.widthPercent == -1.0 {
            self.CalculateWallHitInfo();
        }
        return self.wallType;
    }

    pub fn GetWidthPercent(&mut self) -> f64 {
        if self.widthPercent == -1.0 {
            self.CalculateWallHitInfo();
        }
        return self.widthPercent;
    }

    pub fn GoToNextHit(&mut self) {
        let nextX = self.ray.NextXIntrscPoint(self.hitPoint);
        let nextY = self.ray.NextYIntrscPoint(self.hitPoint);

        let distNextX = self.ray.DistToPoint(nextX);
        let distNextY = self.ray.DistToPoint(nextY);

        if distNextX < distNextY {
            self.hitPoint = nextX;
            self.hitTile += self.ray.xDirVec;
        } else if distNextY < distNextX {
            self.hitPoint = nextY;
            self.hitTile += self.ray.yDirVec;
        } else {
            self.hitPoint = self.ray.direction;
            self.hitTile += self.ray.xDirVec + self.ray.yDirVec;
        }

        self.ClearWallHitInfo();
    }

    pub fn GetNextHit(mut self) -> Self {
        self.GoToNextHit();
        return self;
    }

    pub fn GoToNextCenterHit(&mut self) {
        assert!(GetDecimal(self.hitPoint.x()) == 0.0 || GetDecimal(self.hitPoint.y()) == 0.0);
        let mut vecToCenter: Vec2 = Vec2::default();
        if self.GetWallType() == wallType_t::VERTICAL {
            vecToCenter.SetX(self.ray.xDir as i32 as f64 / 2.0);
            vecToCenter.SetY(self.ray.yDir as i32 as f64 * self.ray.yStep / 2.0);
        } else {
            vecToCenter.SetX(self.ray.xDir as i32 as f64 * self.ray.xStep / 2.0);
            vecToCenter.SetY(self.ray.yDir as i32 as f64 / 2.0);
        }

        *self = RayCursor::New(self.ray,self.hitPoint + vecToCenter);
        self.CalculateWallHitInfo();
    }

    pub fn GetNextCenterHit(mut self) -> Self {
        self.GoToNextCenterHit();
        return self;
    }

    pub fn GetDistToHitPoint(&self) -> f64 {
        self.ray.DistToPoint(self.hitPoint)
    }
}
