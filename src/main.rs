mod utils;
use utils::utils::Vec2D;
use utils::utils::Vec2;

fn main() {

    let v1: Vec2 = Vec2D::new(1.0,2.8);
    let v2: Vec2 = Vec2D::new(1.5, -2.0);
    let v3 = v1 - v2;

    println!("{:?}", v3);
    println!("{}", Vec2D::new(1,1) == Vec2D::new(0,1));
}
