fn main() {
   
any1(&100);
any1(&"Hello World");
any1(&12312.123);

let r1 = Rect::new(10.23,12.45);
any1(&r1);
any2(&r1);
any2(&r1);
}

trait universal{} // This trait does not have any definitions

impl<T> universal for T{}

fn any1<T:universal+std::fmt::Display>(d:&T){
    println!("Value:{}",*d);
}

fn any2<T>(d:&T) where T:universal+std::fmt::Display{
    println!("Value:{}",*d);
}


struct Rect{
    l:f32,
    b:f32
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        Self { l: l, b: b }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.l, self.b)
    }
}