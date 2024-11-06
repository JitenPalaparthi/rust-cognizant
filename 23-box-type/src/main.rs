fn main() {

    let mut b1: Box<i32> = Box::new(100);

    let mut b2: Box<i32> = Box::new(200);
   // let v1 = *b1;

   let b3 = *b1 + *b2;

let c = add(b1,b2);
let a = 10;
let b=20;
add(Box::new(10),Box::new(20));
//add(&10,&20);

add(Box::new(a),Box::new(b));
}


fn add(b:Box<i32>,c:Box<i32>)->Box<i32>{
   let a = *b+ *c;

   return Box::new(a);
}


fn add_as_i64(b:Box<i32>,c:Box<i32>)->Box<i64>{
    let a = *b+ *c;
 //let a:i64 = (* b + *c) as i64;
    return Box::new(a as i64);
 }