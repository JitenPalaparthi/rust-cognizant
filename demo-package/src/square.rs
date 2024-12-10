pub mod shape{
    pub struct Square(f32);
     impl Square{
        pub fn new(s:f32)->Self{
            return Self(s);
        }

        pub fn area(&self)->f32{
            self.0 *self.0
        }

        pub fn perimeter(&self)->f32{
           4.0 * self.0
        }
    }
}