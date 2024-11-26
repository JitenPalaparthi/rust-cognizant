use std::ops::Add;
fn main() {
    println!("Hello, world!");

    let s1= add(10,20);
    let s2= add(12.23,13.45);
    let s3= add(10 as i8, 12 as i8);
    let r1 = Rect::new(10.12,13.45);
    let r2= Rect::new(123.23, 123.23);

    let s4 = add(r1, r2);

    println!("s1:{}",s1);
    println!("s2:{}",s2);
    println!("s3:{}",s3);
    println!("s4:{:?}",s4);

}

trait universal{}
impl<T> universal for T{}

// fn add<T:universal+Add>(a:T,b:T)->T{
//     a+b
// }

fn add<T>(a:T,b:T)->T where T:universal+Add<Output=T>+Copy{
    a+b
}

#[derive(Copy,Clone,Debug)]
struct Rect{
    l:f32,
    b:f32
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        Self { l: l, b: b }
    }
}


//impl Copy for Rect{}

impl Add for Rect{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            l: self.l + other.l,
            b: self.b + other.b,
        }
    }
}