fn main() {
    
    let n1= number;
    let r1=n1.calc(100.12, 200.12);

    println!("add:{:.2} sub:{:.2} mul:{:.2}",r1.0,r1.1,r1.2);
    let f1 = float{};

    let r2= f1.add(12.12, 14.34);

}

trait Calc {
    type Output;
   // fn add(&self,a: f32, b: f32) -> Self::Output;
    fn calc(&self,a:f32,b:f32)-> Self::Output;
    // fill with sub, mul , div
}

trait Eval<T> {
    fn add(&self,a: f32, b: f32) -> T;
}

struct number; // This is a touple structure

impl Calc for number{
    type Output = (f64,f64,f64);
    // fn add(&self,a: f32, b: f32) -> f64{
    //     (a +b) as f64
    // }
    fn calc(&self,a:f32,b:f32)-> Self::Output {
        ((a+b)as f64,(a-b)as f64,(a*b)as f64)
    }
}

struct float;

impl Eval<f64> for float{
    fn add(&self,a: f32, b: f32) -> f64{
        (a +b) as f64
    }
}