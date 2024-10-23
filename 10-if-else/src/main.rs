

fn main() {

    // let age:u8 = 18;
    // let gender:char='f';

    let num = 100;

    let ok:bool= (100>0 && true) || (false && true)  && true;
    //              (true && true) || (false) && true
    //              true || false
    //              true

    println!("ok:{}",ok);

    if num%2==0{
        println!("{} is even",num);
    }else{
        println!("{} is odd",num);
    }

    // use if else as an expression

    let iseven: u8 = if num%2==0{
        1
    }else{
        0
    };
println!("is even:{}",iseven!=0);

}
