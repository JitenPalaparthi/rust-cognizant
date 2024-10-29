fn main() {
    // let k1 = &mut 100;
    // println!("address of k1:{:p}",k1);
    // *k1 = 200;
    // println!("address of k1:{:p}",k1);

    let mut num1: Option<i32> = Some(100);
    let mut num2: Option<i32>= Some(30);

    
    //let sum = num1 + num2;
    let mut n1 = 0;
    let mut n2 =0;
    match num1{
        Some(value)=>{
            n1 = value;
        }
        None=>{
            n1 =0;
        }
    }
    match num2{
        Some(value)=>{
            n2 = value;
        }
        None=>{
            n2 =0;
        }
    }
    let sum1 = n1+n2;
    println!("Sum1:{}",sum1);

    let sum2 = num1.unwrap_or_default()+num2.unwrap_or_default();
    println!("Sum2:{}",sum2);
    num2 = None;
    let sum2 = num1.unwrap_or(0)+num2.unwrap_or(0);
    println!("Sum2:{}",sum2);
    
}

// There is no null or nil
// 