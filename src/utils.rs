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
        CLOCKWISE = -1,

        #[default]
        NONE = 0
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

    #[derive(Debug, Default, PartialEq)]
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

    /*
    =================================================
        Vec2D methods
    =================================================
    */

    impl<T: ValidVecElement> Vec2D<T> {
        pub fn New(e1: T, e2: T) -> Self {
            return Vec2D { e: [e1, e2] }
        }

        pub fn x(&self) -> T {
            return self.e[0];
        }

        pub fn y(&self) -> T {
            return self.e[1];
        }
    }

    impl Vec2D<f64> {
        pub fn LengthSquared(&self) -> f64 {
            return self.e[1].powi(2) + self.e[1].powi(2);
        }

        pub fn Length(&self) -> f64 {
            return self.LengthSquared().sqrt();
        }

        pub fn Rotate(&self, rad: f64) -> Self {
            return Vec2D::New(self.e[0]*rad.cos() - self.e[1]*rad.sin(), self.e[0]*rad.sin() + self.e[1]*rad.cos());
        }

        pub fn UnitVector(&self) -> Self {
            let vectorMag = self.Length();
            return Vec2D::New(self.e[0]/vectorMag, self.e[1]/vectorMag);
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
            return &self.e[index];
        }
    }

    impl<T: ValidVecElement> IndexMut<usize> for Vec2D<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            return &mut self.e[index];
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
            return Vec2D::New(-self.e[0], - self.e[1]);
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
            return Vec2D::New(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1]);
        }
    }

    impl<T: ValidVecElement> Sub for Vec2D<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            return Vec2D::New(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1]);
        }
    }

    /*
    =================================================
        Free functions
    =================================================
    */

    pub fn Dot(v1: &Vec2D<f64>, v2: &Vec2D<f64>) -> f64 {
        return v1.x()*v2.x() + v1.y()*v2.y();
    }
}

pub mod ray {
    use crate::utils::vec2d::{iVec2, Point2, Vec2};
    use crate::utils::conventions::*;

    #[derive(Default)]
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

    impl Ray {
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

    }
}

