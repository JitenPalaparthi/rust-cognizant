pub mod cuboid;
pub mod rect;
pub mod square;

pub trait IShape {
     fn area(&self)->f32;
     fn perimeter(&self)->f32;
     fn what(&self)->String;
}

pub fn why(){
     println!("Just to check how this work");
 }