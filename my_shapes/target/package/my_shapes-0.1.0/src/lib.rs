pub mod shapes;

pub fn print_shape(ishape:&dyn shapes::IShape){
    println!("Area of {} is :{:.2}",ishape.what(),ishape.area());
    println!("Perimeter of {} is :{:.2}",ishape.what(),ishape.perimeter());
}