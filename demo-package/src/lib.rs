// src/lib.rs

 mod internal {
    pub fn internal_greet() {
        println!("Hello World")
    }
}
pub mod greeting {
    use super::internal;
    // pub mod internal{
    pub fn greet() {
        internal::internal_greet();
    }
    pub fn greet_msg(msg: &str) {
        println!("{}", msg);
    }
}

pub mod shapes;
pub mod square;

// create a struct rect
// it should be from a module called shapes
// make the module public
// make the type public
// create a new public function
// implement two methods called area and perimeter
// call them in main
// do it in lib.rs only
// difference between super vs crate