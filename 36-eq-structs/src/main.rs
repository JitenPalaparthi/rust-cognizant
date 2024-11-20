use std::{clone, fmt::format};

//type int = i32; // not creating a new type instead aliasing an existing type


struct int(i32);

impl int{
    fn to_a_string(&self)->String{
        format!("{}",self.0)
    }
}

fn main() {

let i1 = int(100);
   let s1= i1.to_a_string();
   println!("{}",s1);
    //et l1:int = 10;

    let r1 = Rect::new(10.1223123, 12.12);
    
    let r2 = Rect::new(10.1223433,12.12);

    let r3 = Rect::new(13.14,15.65);
    if r1 == r2{
        println!("r1 and r2 are equal");
    }else{
        println!("r1 and r2 are not equal");
    }


    if r1 == r3{
        println!("r1 and r3 are equal");
    }else{
        println!("r1 and r3 are not equal");
    }

    let r4 = r1.clone();
    let r5 = r1; // the reason you can do this is copy trait is implemented

    println!("{:?}",r1);

    let s1 = String::new();
    //s1.into()
    
}

#[derive(Debug)]
struct Rect {
    l: f32,
    b: f32,
}

impl PartialEq for Rect{
    fn eq(&self,other:&Self)->bool{
        if self.l.round()==other.l.round() && self.b.round() == other.b.round(){
           return true;
        }
        return false;
    }
}

impl Copy for Rect{}

impl Clone for Rect{
    fn clone(&self) -> Self {
        *self
    }
}

impl Rect{
    fn new(l:f32,b:f32)->Rect{
        Rect{l:l,b:b}
    }
}
impl Rect{
    fn area(&self)->f32{
        self.l * self.b
    }
    fn perimeter(&self)->f32{
        2.0 * (self.l+self.b)
    }
}
 
// round floor,ceil trunc, fract are available for f32 and f64
 
// 12.123123123123123
// 12.123123342342342

// into and from traits

