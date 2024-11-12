fn main() {
let a: i32 = 100; // what is the lifetime of a ? where does a store? 
{
    let b: i32 = 200;

    let str1: String = "hello World".to_string();
}
let str2: String = "hello Universe".to_string();

let mut box3: &mut Box<i32> = &mut Box::new(0);
let mut box5 = Box::new(0);
let mut box1 = Box::new(100);
{
let box2 = &mut box1;
    let mut box4 = Box::new(1000);
    box5 = box4.clone(); // clone values to a new memory
   // box3 = &mut box4;
    println!("{}",*box3);
}
println!("Box3:{}", *box3);
println!("Box5:{}",*box5);
}
