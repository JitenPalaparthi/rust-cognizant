fn main() {

   let mut r1 = Rect{
        length:12.34,
        bredth:13.45,
        area:0.0,
        perimter:0.0,
    };

    let a1 = r1.area_of();
    let p1 = r1.perimeter_of();
    println!("Area of Rect r1:{:.2}",a1); // only prints two digits after .
    println!("Perimeter of Rect r1:{}",p1);
}

#[derive(Debug)] // this helps you to print the debug print
struct Rect{
    length:f32,
    bredth:f32,
    area:f32,
    perimter:f32
}

impl Rect{
    fn area_of(&mut self)->f32{
        self.area=self.length * self.bredth;
        self.area
    }
    fn perimeter_of(&mut self)->f32{
        self.perimter=2.0 * (self.length+self.bredth);
        return self.perimter;
    }
}

