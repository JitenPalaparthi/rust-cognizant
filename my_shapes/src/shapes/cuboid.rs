#[derive(Debug)]
pub struct Cuboid {
    length: f32,
    breadth: f32,
    height:f32,
}

impl Cuboid {
    pub fn new(l: f32, b: f32,h:f32) -> Self {
         Self {
            length: l,
            breadth: b,
            height:h,
        }
    }
}
impl crate::shapes::IShape for Cuboid{ // calling from crate, which is root
     fn area(&self) -> f32 {
         self.length * self.breadth * self.height
    }

     fn perimeter(&self)->f32{
         2.0 * (self.length+self.breadth+self.height)
    }
    fn what(&self)->String {
         "Cuboid".to_string()
    }
}
