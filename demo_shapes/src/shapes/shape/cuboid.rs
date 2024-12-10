#[derive(Debug)]
pub struct Cuboid {
    length: f32,
    breadth: f32,
    height:f32,
}

impl Cuboid {
    pub fn new(l: f32, b: f32,h:f32) -> Self {
        return Self {
            length: l,
            breadth: b,
            height:h,
        };
    }
}
impl super::IShape for Cuboid{
     fn area(&self) -> f32 {
        return self.length * self.breadth * self.height;
    }

     fn perimeter(&self)->f32{
        return 2.0 * (self.length+self.breadth+self.height);
    }
    fn what(&self)->String {
        return "Cuboid".to_string();
    }
}
