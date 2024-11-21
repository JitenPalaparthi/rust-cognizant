use std::ops::{Add, Mul, Sub};

fn main() {
    let p1 = &Point::new(10, 20);
    let p2 = &Point::new(30, 40);
    let p4 = *p1 - *p2;
    let p3 = *p1 + *p2;
    println!("sub:{:#?}", p4);
    println!("add:{:#?}", p3);
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}
#[derive(Copy, Clone, Debug)]
struct MyPoint {
    a: i32,
    b: i32,
}

impl Add for Point {
    type Output = MyPoint;
    fn add(self, other: Self) -> MyPoint {
        MyPoint {
            a: self.x + other.x,
            b: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// implement Mul type for Point
// also once multiplied the result should be in another type
// which must be in x:f32 and y:f32 format
/* output
        struct MPoint{
        x:f32,
        y:f32
        }
*/
