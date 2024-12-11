#[derive(Debug)]
pub struct Square(f32);
impl Square{
   pub fn new(s:f32)->Self{
        Self(s)
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
     "Square".to_string()
   }
}
