#[derive(Debug)]
pub struct Square(f32);
impl Square{
   pub fn new(s:f32)->Self{
       return Self(s);
   }
}
impl super::IShape for Square{
    fn area(&self)->f32{
       self.0 *self.0
   }

    fn perimeter(&self)->f32{
      4.0 * self.0
   }
   fn what(&self)->String {
       return "Square".to_string();
   }
}

pub fn why(){
    println!("Just to check how this work");
}