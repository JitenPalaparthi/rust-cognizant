//use demo_package::{greet,greet_msg};
fn main() {
    // greet();
    // greet_msg("Hello Cognizent");
    use  demo_package::greeting::{greet,greet_msg}; // can also call the path inside
        greet();
        greet_msg("Hello Universe");
    demo_package::greeting::greet_msg("Hello Cognizant");
   //demo_package::internal::internal_greet();
   use demo_package::shapes::rect::rectangle::Rect;
   let r1= Rect::new(10.12, 12.12);
   let a1=r1.area();
   println!("Area of r1:{:.2}",a1);
   let p1 = r1.perimeter();
   println!("Perimeter of r1:{}",p1);

   use demo_package::square::shape::Square;

   let s1 = Square::new(25.34);
   let a1 = s1.area();
   println!("Area of s1:{:.2}",a1);

   let p1 = s1.perimeter();
   println!("Perimeter of s1:{}",p1);

}

// creating a crate outside of main and calling it..
// writing unit tests
// publish a crate to crates.io

// restful service using actix-web
// writing simple crud operations to the database using sqlx
