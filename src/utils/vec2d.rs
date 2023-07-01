use std::ops::{Add, Sub, AddAssign, SubAssign, Index, IndexMut, Neg, Mul};

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

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2D::New(self.e[0] * scalar, self.e[1] * scalar)
    }
}

/*
=================================================
    Free functions
=================================================
*/

pub fn Dot(v1: Vec2D<f64>, v2: Vec2D<f64>) -> f64 {
    v1.x()*v2.x() + v1.y()*v2.y()
}
