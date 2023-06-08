pub mod utils {
    use std::ops::{Add, Sub, AddAssign, SubAssign};

    //#[derive(Copy)]
    #[derive(Debug, PartialEq)]
    pub struct Vec2D<T: Copy + Add<Output = T> + Sub<Output = T>> {
        pub e: [T; 2]
    }

    pub type Vec2    = Vec2D<f64>;
    pub type iVec2   = Vec2D<i32>;
    pub type Point2  = Vec2;
    pub type iPoint2 = iVec2;
    pub type Pixel   = iVec2;

    impl<T: Copy + Add<Output = T> + Sub<Output = T>> Vec2D<T> {
        pub fn new(e1: T, e2: T) -> Self {
            return Vec2D { e: [e1, e2] }
        }
    }

    impl<T: Copy + Add<Output = T> + Sub<Output = T>> Add for Vec2D<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            return Vec2D::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1]);
        }
    }

    impl<T: Copy + Add<Output = T> + Sub<Output = T>> Sub for Vec2D<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            return Vec2D::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1]);
        }
    }
}