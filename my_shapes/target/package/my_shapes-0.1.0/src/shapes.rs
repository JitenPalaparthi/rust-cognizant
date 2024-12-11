pub mod cuboid;
pub mod rect;
pub mod square;

pub trait IShape {
     fn area(&self)->f32;
     fn perimeter(&self)->f32;
     fn what(&self)->String;
}
