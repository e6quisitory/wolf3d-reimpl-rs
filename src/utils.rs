pub mod conventions {

    pub const PI: f64                 = std::f64::consts::PI;
    pub const TEXTURE_PITCH: i32      = 64;
    pub const TRANSPARENCY_COLOR: u32 = 0xFF980088;

    #[derive(Default, Copy, Clone)]
    pub enum xDir_t {
        EAST = 1,
        WEST = -1,

        #[default]
        NONE = 0
    }

    #[derive(Default, Copy, Clone)]
    pub enum yDir_t {
        NORTH = 1,
        SOUTH = -1,

        #[default]
        NONE = 0
    }

    #[derive(Default, Copy, Clone)]
    pub enum swivelDir_t {
        COUNTER_CLOCKWISE = 1,
        CLOCKWISE         = -1,

        #[default]
        NONE = 0
    }

}

pub mod misc_math {

    use crate::utils::conventions::PI;

    pub fn GetDecimal(f: f64) -> f64 {
        let f_abs = f.abs();
        return f_abs - f_abs.floor();
    }

    pub fn IsInteger(f: f64) -> bool {
        if GetDecimal(f) == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn DegreesToRadians(degrees: f64) -> f64 {
        degrees*PI/180.0
    }

}

pub mod vec2d {

    use std::ops::{Add, Sub, AddAssign, SubAssign, Index, IndexMut, Neg};

    /*
    =================================================
        Trait bounds for a valid Vec2D element
    =================================================
    */

    pub trait ValidVecElement: Copy + Add<Output = Self> + Sub<Output = Self> + AddAssign + SubAssign + Neg<Output = Self> {}
    impl<T: Copy + Add<Output = Self> + Sub<Output = Self> + AddAssign + SubAssign + Neg<Output = Self>> ValidVecElement for T {}

    /*
    =================================================
        Vec2D struct definition
    =================================================
    */

    #[derive(Debug, Default, PartialEq, Copy, Clone)]
    pub struct Vec2D<T: ValidVecElement> {
        pub e: [T; 2]
    }

    /*
    =================================================
        Type aliases for Vec2D
    =================================================
    */

    pub type Vec2    = Vec2D<f64>;
    pub type iVec2   = Vec2D<i32>;
    pub type Point2  = Vec2;
    pub type iPoint2 = iVec2;
    pub type Pixel   = iVec2;

    impl From<Point2> for iPoint2 {
        fn from(value: Point2) -> Self {
            iPoint2::New(value.e[0] as i32, value.e[1] as i32)
        }
    }

    impl From<iPoint2> for Point2 {
        fn from(value: iPoint2) -> Self {
            Point2::New(value.e[0] as f64, value.e[1] as f64)
        }
    }

    /*
    =================================================
        Vec2D methods
    =================================================
    */

    impl<T: ValidVecElement> Vec2D<T> {
        pub fn New(e1: T, e2: T) -> Self {
            Vec2D { e: [e1, e2] }
        }

        pub fn x(self) -> T {
            self.e[0]
        }

        pub fn SetX(&mut self, newX: T) {
            self.e[0] = newX;
        }

        pub fn y(self) -> T {
            self.e[1]
        }

        pub fn SetY(&mut self, newY: T) {
            self.e[1] = newY;
        }
    }

    impl Vec2 {
        pub fn LengthSquared(&self) -> f64 {
            self.e[1].powi(2) + self.e[1].powi(2)
        }

        pub fn Length(&self) -> f64 {
            self.LengthSquared().sqrt()
        }

        pub fn Rotate(&self, rad: f64) -> Self {
            Vec2D::New(self.e[0]*rad.cos() - self.e[1]*rad.sin(), self.e[0]*rad.sin() + self.e[1]*rad.cos())
        }

        pub fn UnitVector(&self) -> Self {
            let vectorMag = self.Length();
            Vec2D::New(self.e[0]/vectorMag, self.e[1]/vectorMag)
        }
    }

    /*
    =================================================
        lvalue operators
    =================================================
    */

    impl<T: ValidVecElement> Index<usize> for Vec2D<T> {
        type Output = T;
        fn index(&self, index: usize) -> &Self::Output {
            &self.e[index]
        }
    }

    impl<T: ValidVecElement> IndexMut<usize> for Vec2D<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.e[index]
        }
    }

    impl<T: ValidVecElement> AddAssign for Vec2D<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.e[0] += rhs[0];
            self.e[1] += rhs[1];
        }
    }

    impl<T: ValidVecElement> SubAssign for Vec2D<T> {
        fn sub_assign(&mut self, rhs: Self) {
            self.e[0] -= rhs[0];
            self.e[1] -= rhs[1];
        }
    }

    impl<T: ValidVecElement> Neg for Vec2D<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Vec2D::New(-self.e[0], -self.e[1])
        }
    }

    /*
    =================================================
        rvalue operators
    =================================================
    */

    impl<T: ValidVecElement> Add for Vec2D<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Vec2D::New(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1])
        }
    }

    impl<T: ValidVecElement> Sub for Vec2D<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vec2D::New(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1])
        }
    }

    /*
    =================================================
        Free functions
    =================================================
    */

    pub fn Dot(v1: &Vec2D<f64>, v2: &Vec2D<f64>) -> f64 {
        v1.x()*v2.x() + v1.y()*v2.y()
    }

}

pub mod ray {

    use crate::utils::vec2d::{iVec2, Point2, Vec2};
    use crate::utils::conventions::*;
    use crate::utils::misc_math::IsInteger;

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

}

pub mod dda {
    use crate::utils::ray::*;
    use crate::utils::vec2d::{iPoint2, Point2, Vec2};
    use crate::utils::misc_math::*;

    /*
    =========================================================
        Relevant type definitions
    =========================================================
    */

    #[derive(Default, Copy, Clone, PartialEq)]
    enum wallType_t {
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

        fn GetWallType(&mut self) -> wallType_t{
            if self.widthPercent == -1.0 {
                self.CalculateWallHitInfo();
            }
            return self.wallType;
        }

        fn GetWidthPercent(&mut self) -> f64 {
            if self.widthPercent == -1.0 {
                self.CalculateWallHitInfo();
            }
            return self.widthPercent;
        }

        /* Public methods */

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
            self.GoToNextHit();
            return self;
        }

        pub fn GetDistToHitPoint(&self) -> f64 {
            self.ray.DistToPoint(self.hitPoint)
        }
    }

}

pub mod mapCSV {
    use std::error::Error;
    use std::fs::File;

    use csv::ReaderBuilder;
    use ndarray::Array2;

    pub fn parseCSV(path: &str) -> Result<Array2<i32>, Box<dyn Error>> {
        // Open the file
        let file = File::open(path)?;

        // Build CSV reader with ',' as delimiter and flexible number of fields
        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .flexible(true)
            .has_headers(false)  // Do not treat first row as headers
            .from_reader(file);

        let mut array_data = Vec::new();

        // Read each record
        for result in rdr.records() {
            let record = result?;

            // Skip empty rows
            if record.iter().next().is_none() {
                continue;
            }

            let mut row = Vec::new();
            for field in record.iter() {
                // Convert each field to an integer, using 0 if the field is empty
                let value: i32 = field.parse().unwrap_or(0);
                row.push(value);
            }

            array_data.push(row);
        }

        // Find maximum row length to handle jagged arrays
        let max_len = array_data.iter().map(|row| row.len()).max().unwrap_or(0);

        // Normalize rows to have equal length
        for row in &mut array_data {
            let diff = max_len - row.len();
            if diff > 0 {
                row.extend(vec![0; diff]);
            }
        }

        // Convert the data to a 2D array
        let mut array: Array2<i32> = ndarray::Array::from_shape_vec((array_data.len(), max_len), array_data.concat())
            .expect("Error converting to 2D array");

        // Adjust perimeter values to be 1 if they are 0
        let (max_i, max_j) = (array.nrows() - 1, array.ncols() - 1);
        for i in 0..=max_i {
            for j in 0..=max_j {
                if i == 0 || j == 0 || i == max_i || j == max_j {
                    if array[[i, j]] == 0 {
                        array[[i, j]] = 1;
                    }
                }
            }
        }

        Ok(array)
    }
}