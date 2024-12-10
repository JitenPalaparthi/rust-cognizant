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
    //}
}
