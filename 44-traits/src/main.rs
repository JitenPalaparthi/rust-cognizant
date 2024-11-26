fn main() {
    println!("Hello, world!");

    let s1= add(10,20);
    let s2= add(12.23,13.45);
    let s3=  add(10 as i8, 12 as i8);
}

trait universal{}
impl<T> universal for T{}

fn add<T:universal>(a:T,b:T)->T{
    a+b
}