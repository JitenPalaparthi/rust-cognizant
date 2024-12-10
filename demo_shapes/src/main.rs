use demo_shapes::shape_of;
use demo_shapes::shapes::shape::rect::Rect;
use demo_shapes::shapes::shape::square::Square;
use demo_shapes::shapes::shape::cuboid::Cuboid;
fn main() {
    let r1 = Rect::new(100.1, 110.2);
    let s1 = Square::new(12.34);
    let c1 = Cuboid::new(10.3,14.5,16.4);
    shape_of(&r1);
    shape_of(&s1);
    shape_of(&c1);
}
