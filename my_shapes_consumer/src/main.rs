
use my_shapes::shapes::{rect,square,cuboid};
fn main() {
    let r1= rect::Rect::new(10.12, 13.12);
    let c1= cuboid::Cuboid::new(10.12, 13.12,14.34);
    let s1 = square::Square::new(15.45);

    my_shapes::print_shape(&r1);
    println!();
    my_shapes::print_shape(&c1);
    println!();
    my_shapes::print_shape(&s1);
}
