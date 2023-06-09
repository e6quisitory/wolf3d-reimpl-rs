pub mod vec2d {
    use std::ops::{Add, Sub, AddAssign, SubAssign, Index, IndexMut};

    /*
    =================================================
        Trait bounds for a valid Vec2D element
    =================================================
    */

    pub trait ValidVecElement: Copy + Add<Output = Self> + Sub<Output = Self> {}
    impl<T: Copy + Add<Output = Self> + Sub<Output = Self>> ValidVecElement for T {}

    /*
    =================================================
        Vec2D struct definition
    =================================================
    */

    #[derive(Debug, PartialEq)]
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
        pub fn new(e1: T, e2: T) -> Self {
            return Vec2D { e: [e1, e2] }
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


    /*
    =================================================
        rvalue operators
    =================================================
    */

    impl<T: ValidVecElement> Add for Vec2D<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            return Vec2D::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1]);
        }
    }

    impl<T: ValidVecElement> Sub for Vec2D<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            return Vec2D::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1]);
        }
    }
}