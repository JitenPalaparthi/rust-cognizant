pub mod shapes;

use shapes::shape::IShape;

pub fn shape_of(ishape: &dyn IShape){
    println!("area of {} is :{:.2}",ishape.what(),ishape.area());
    println!("perimeter of {} is :{:.2}",ishape.what(),ishape.perimeter());
}