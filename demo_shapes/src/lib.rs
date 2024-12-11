pub mod shapes;
pub mod greetings;

use shapes::shape::IShape;

pub fn shape_of(ishape: &dyn IShape){
    println!("area of {} is :{:.2}",ishape.what(),ishape.area());
    println!("perimeter of {} is :{:.2}",ishape.what(),ishape.perimeter());
   // greetings::Greet("Hello Worlld");
}



/*

lib.rs
    shapes.rs
        shapes/rect.rs
        shapes/square.rs
        shapes/cuboid.rs
    greetings.rs
*/