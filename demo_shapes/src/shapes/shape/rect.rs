use crate::shapes::shape::square;

use super::square::why;


#[derive(Debug)]
pub struct Rect {
    length: f32,
    breadth: f32,
}


impl Rect {
    pub fn new(l: f32, b: f32) -> Self {
        return Self {
            length: l,
            breadth: b,
        };
    }
}
impl super::IShape for Rect{
     fn area(&self) -> f32 {
        return self.length * self.breadth;
    }

     fn perimeter(&self)->f32{
        return 2.0 * (self.length+self.breadth);
    }
    fn what(&self)->String {
        return "Rect".to_string();
    }
}

pub fn whyr(){
   //use super::square::why;
  // use crate::shapes::shape::why;
  // use super::why;
  //  why();
  //use super::super::super::greetings::Greet;
  use crate::greetings::Greet;
  Greet("Hello World");
  square::why();
  super::square::why();
  super::super::super::shapes::shape::square::why();
}