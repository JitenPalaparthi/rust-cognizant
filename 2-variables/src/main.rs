static mut count:u32 = 0; // data segment 
const V:f32 = 1.0; 
fn main() {

    let mut v1 = 1.5;

    v1 = 1.5 * 1.5 ; // it is evaluated at runtime

    const PI:f32 = 3.14 * (V + 1.0) / 5.0  ; // this is an expression

    println!("Hello, world!");
    let mut num1:usize = 1231; //8 byte
    num1 = 32324324234;//num1 * num1; //8 bytes
    println!("PI:{} num1:{}",PI,num1);

    { // a separate stackframe is created
        let num2:u8 = 123;
        println!("num2:{}",num2);
    }

    let mut name: &str = "Cognizant"; // name does not own the data "Congnizant"
    name = "Cognizant India Private Ltd"; // 27 bytes

    println!("Company:{}",name);

    
    // String name ="Jiten";
    // name = "Jiten P";
unsafe{ // I am going to take the risk so unsafe code
    count +=1;
    count +=10;
    println!("Count:{}",count);
}

}

// &str is immutable 

/*
str:
  ptr         // 8 bytes
  size/length // 8 bytes
*/