fn main() {

    let day:u8 =10;;

    match day{
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("Noday")
    }

    let num: i32 = 19;

    match num{
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime number:{}",num),
        13..=19 => {
            println!("teen number");
        },
        _ => println!("undefined pattern")
    }

    let num = 13;

    match num{
        x if x%2 == 0 && true  => {
            println!("{} is even number",x)
        },
        x if x%2 !=0 => println!("{} is odd number",x),
        // if num % 2==0 => println!("{}",num),
        _ => { println!("something else")}
    }

}
// There is no null in rust
// Excepting handling is different in rust. Errors are just values
// 


