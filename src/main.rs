mod utils;
use utils::vec2d::*;

fn main() {

    let v1 = Vec2D::New(1.0,2.8);
    let v2 = Vec2D::New(1.5, -2.0);
    let mut v3 = v1 - v2;

    let e1 = v3[0];

    println!("{:?}", v3);
    println!("{}", Vec2D::New(1,1) == Vec2D::New(0,1));
    println!("{}", e1);

    v3[0] = 24.3;

    println!("{:?}", v3);

    let v6 = Vec2D::New(1.0, 2.0);
    let v7 = Vec2D::New(3.0, 4.0);
    let v8 = v7 - v6;
    println!("{:?}", v8.x());
    println!("{:?}", v8.y());
    println!("{:?}", v8);
    println!("{:?}", v8.LengthSquared());
    println!("{:?}", v8.Length());
    println!("{:?}", v8.Rotate(3.14159/2.0));
    println!("{:?}", v8.UnitVector());
    println!("{:?}", Dot(&v3, &v8));
    println!("{:?}", -v8);

}
