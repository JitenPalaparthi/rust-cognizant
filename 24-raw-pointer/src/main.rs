
use std::mem;
static mut GB: i32 = 100; // data segment
fn main() {


    let mut num1: i32 = 100;

    let raw_ptr: *mut i32 = &mut num1;

    unsafe {
        *raw_ptr = 200; //dereferencing
    }

    println!("raw_pointer address:{:p} num1 address:{:p}", raw_ptr, &num1);

    unsafe {
        println!("raw_pointer:{}", *raw_ptr);
    }

    println!("num1:{}", num1);
    unsafe {
        GB += 1;
        println!("Global GB:{}",GB);
    }
  

  let mut b1:Box<i32>=Box::new(100);
  *b1 = 200;

  println!("b1:{:p}",b1);
  println!("address of b1:{:p}",&b1);
  let mut b2: &mut Box<i32> = &mut b1; // transfer not a copy
  //*b1= 300;
  //b2 = &mut b1;
  //*b1=400;
  *b2= Box::new(20000);
  println!("b2:{:p}",b2);
  println!("address of b2:{:p}",&b2);

  println!("b1:{}",*b1);
println!("Size of box:{:?}",mem::size_of::<Box<String>>());    // 8
println!("Size of i32:{:?}",mem::size_of::<i32>());            // 4
println!("Size of &str:{:?}",mem::size_of::<&str>());          // 16
println!("Size of String:{:?}",mem::size_of::<String>());      // 24
println!("Size of Vector:{:?}",mem::size_of::<Vec<i32>>());    // 24

let str1="Hello World";


}

/*
Box
ptr: --> address of string  --> string struct -- ptr len cap
len: --> 

*/
