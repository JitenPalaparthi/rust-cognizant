fn main() {
  let r1 = Rect::new(10.12, 12.23);
  let s1 = Square::new(12.23);
 // let k1 = (12,12.13,true);

  call_shape_static(&r1);
  call_shape_static(&s1);
    println!();
  call_shape_dynamic(&r1);
  call_shape_dynamic(&s1);

  //call_shape(k1);
}

// Rect 
// Square

// Area and Perimeter

trait shape{
    fn area(&self)->f32;
    fn perimeter(&self)->f32;
    fn what(&self)->String;
}

fn call_shape_static(s:&impl shape){  
    println!("Calling using static dispatch");
    let a = s.area();
    let p = s.perimeter();
    let w =  s.what();
    println!("Area:{} Perimeter:{} for {}",a,p,w);
}

fn call_shape_dynamic(s: &dyn shape){  
    println!("Calling using dynamic dispatch");
    let a = s.area();
    let p = s.perimeter();
    let w =  s.what();
    println!("Area:{} Perimeter:{} for {}",a,p,w);
}


struct Rect{
    l:f32,
    b:f32,
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        return Rect{l:l,b:b};
    } 
}

impl shape for Rect{
    fn area(&self)->f32{
        return self.l * self.b;
    }
    fn perimeter(&self)->f32{
        return 2.0 * (self.l+self.b)
    }
    fn what(&self)->String{
        return "Rect".to_string();
    }
}

struct Square(f32);

impl Square{
    fn new(s:f32)->Square{
        Self(s)
    }
}

impl shape for Square{
    fn area(&self)->f32 {
        self.0 * self.0
    }
    fn perimeter(&self)->f32 {
        4.0 * self.0
    }
    fn what(&self)->String {
        "Square".to_string()
    }
}

// Trait --> Talk 
// Aninal -> Dog, Cat, Lion
// Implement Talk function for all these tyoes..
// create new function for all thoses and create a general function called talk(ITalk) 

// impl vs dyn
