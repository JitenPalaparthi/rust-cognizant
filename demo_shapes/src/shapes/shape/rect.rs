
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
