fn main() {

    let p1 = Point(10,20);
    println!("{:?}",p1);
    let s1 = p1.sum_of();
    println!("s1:{}",s1);
   // String::new()

  let p2= Point::new(30, 40);
  let p3 = Point::default();

  let s2 = p2.sum_of();
  let s3 = p3.sum_of();

  println!("s2:{}",s2);
  println!("s3:{}",s3);

  let sq1= Square::new(25.25);
  let a1 = sq1.area();
  let p1 = sq1.perimeter();

  println!("Area of sq1:{}",a1);
  println!("Perimeter of sq1:{}",p1);
}

#[derive(Debug)]
struct Point(i32,i32); // touple struct

impl Point{
    fn new(a:i32,b:i32)->Self{
        Self(a,b)
    }

    fn default()->Point{
        Point(1,1)
    }

    fn sum_of(&self)->i32{
        return self.0 + self.1
    }
}

struct Square(f32); // touple struct


impl Square{
    fn new(s:f32)->Self{
        Self(s)
    }

    // fn new(s:f32)->Square{
    //     Square(s)
    // }

    fn area(&self)->f32{
        self.0 * self.0
    }
   fn perimeter(&self)->f32{
        4 as f32 * self.0
   }

}
