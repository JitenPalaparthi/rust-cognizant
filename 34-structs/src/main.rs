fn main() {

    let _unit = Unit;
    println!("_unit={:?}",_unit);

    let u1 = Unit::new();
    u1.what();

   let  a = A::new();
   let b= B::new();
   let a1 =a.add(123,123);
   let b1 = b.add(123.123, 12.23);

   println!("a1:{}\nb1:{}",a1,b1);
}

#[derive(Debug)]
struct Unit; // a struct without any field

impl Unit{
    fn new()->Unit{
        Unit
    }

    fn what(&self){
        println!("I am unit struct.Currently do nothing")
    }
}

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

impl A{
    fn new()->A{
        A
    }
    fn add(&self, a:i32,b:i32)->i32{
        a+b
    }
}

impl B{
    fn new()->B{
        B
    }
    fn add(&self, a:f32,b:f32)->f32{
        a+b
    }

}