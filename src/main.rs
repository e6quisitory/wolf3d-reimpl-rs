mod utils;
use utils::vec2d::Vec2D;
use utils::vec2d::Vec2;

fn main() {

    let v1: Vec2 = Vec2D::new(1.0,2.8);
    let v2: Vec2 = Vec2D::new(1.5, -2.0);
    let mut v3 = v1 - v2;

    let e1 = v3[0];

    println!("{:?}", v3);
    println!("{}", Vec2D::new(1,1) == Vec2D::new(0,1));
    println!("{}", e1);

    v3[0] = 24.3;

    println!("{:?}", v3);
}
