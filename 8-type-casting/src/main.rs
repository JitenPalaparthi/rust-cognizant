fn main() {

    let num1:u8 = 250;

    let num2:u32 = num1 as u32;

    let ok1:bool = false;

    let num3:u8 = ok1 as u8; // not possible in other programming langauges

    println!("num1:{} num2:{} ok1:{} num3:{}",num1,num2,ok1,num3);

    let pi:f64 = 3.14;

    let num4:usize = pi as usize;

    println!("pi:{} num4:{}",pi,num4);

    let num5:u64=12312312312;// 1011011101110111101111100111111000
    let num6:u8= num5 as u8; //
    // 11111000 -->248

    println!("num5:{} num6:{}",num5,num6);

    let c:char = 'A';

    let num7:u32 = c as u32;
    println!("num7:{}",num7);
    let num8 = 65; 
    let num9 =19000;
    let c2:char = (num8 as u8 )as char;
    let c3:char = num9 as u8 as char;

    if let Some(c4)= char::from_u32(num9){
        println!("char:{}",c4);
    }else{
        println!("unable to conver");
    }

    let str1="65";

    let num1:i64 = str1.parse().expect("Not a valid number");
    //let num1:u8 = str1.parse::<u8>().expect("Not a number");

    println!("num1:{}",num1);
   // let four: Result<u32, std::num::ParseIntError> = "4".parse::<u32>();
    

}

//git clone git@github.com:JitenPalaparthi/rust-cognizant.git

