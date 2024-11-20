fn main() {
    println!("Hello, world!");
    // ! tells that it is a macro
    let num1:i8  = 100;
    let num2:u32  = 1232131;
    let num3:i32  = -12321;
    let num4:isize = 1231231;
    let num5:i64   = 123123123;
    let num6:u128  = 12312312312321312312312312312312133;
    let float1:f32 =12312.123;
    let float2 :f64 =123123123.1231231231;

    let age1 = 39;
    let age2:u8 = 42;

    let float3 = 12312.12; // type inference 

    let sum: i64 = 1231231223432423+1231231231;

    let ok = true;

    let num7:isize =123;

    println!("num1: {} num2:{} num3:{} num4:{} num5:{} num6:{} flaot1: {} float2:{:.4}",num1,num2,num3,num4,num5,num6,float1,float2);
    let sum2 = sumf(231231233,132322312);

    println!("sum2:{}",sum2);
    println!("value of num7:{}",num7);

    let char1:char = 'a';

    let char2 = '界'; // type is inferred based on the value

    println!("char1:{0} char2:{1}",char1,char2);

    

}


fn sumf(i:i32,j:i32)->i64{
    i as i64+ j as i64
}

// does it really start from main? 
// does rust application has runtime? 

// Who does manage the memory?
// How to manage the memory?

// numbers
// i8,i16,i32,i64,i128, isize
// u8,u16,u32,u64,u128, usize 
// f32, f64