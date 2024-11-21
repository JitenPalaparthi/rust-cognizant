use core::num;
use std::ops::Add;
fn main() {
    let l1: empty = empty;
    l1.greet("Hello World");
    l1.sayhi();
}

trait Greet {
    fn greet(&self, msg: &str) { // can implement basic implementation of trait definitions as well
        println!("{}", msg);
    }
    fn sayhi(&self);
}

struct empty;

impl Greet for empty {
    // fn greet(&self,msg:&str){
    //     println!("{}",msg);
    // }
    fn sayhi(&self) {
        println!("Hello");
    }
}
