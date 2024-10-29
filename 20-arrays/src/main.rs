use rand::Rng;
fn main() {
   
   let mut numbers1: [i32; 10] = [10;10];// all elements have the value 1 is given

   println!("numbers:{:?}",numbers1);

   let mut nunmbers2 = [10,20,30,40,50,60,60,70,80,19,100,110];
   println!("numbers:{:?}",nunmbers2);

    // create an array with random numbers

    let mut numbers3:[i32;100]=[0;100];

    let mut rng = rand::thread_rng();

    for n in &mut numbers3{
        *n=rng.gen_range(1..=100);
    }
    println!("numbers={:?}",numbers3);
}
// cargo add rand 
// the package is pulled from crates.io 