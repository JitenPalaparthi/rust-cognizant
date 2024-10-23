fn main() {
    let age: u8 = 18;
    let gender: char = 'F';

    if age >= 18 && (gender == 'f' || gender == 'F') {
        println!("She is eligible for marriage,becasuse age is {}", age);
    } else if age >= 21 && (gender == 'm' || gender == 'M') {
        println!("He is eligible for marriage,becasuse age is {}", age);
    } else {
        println!("Not eligible for marriage");
    }

    let is_eligible = if age >= 18 && (gender == 'f' || gender == 'F') {
        "Yes"
    } else if age >= 21 && (gender == 'm' || gender == 'M') {
        "Yes"
    } else {
        "No"
    };

    println!("is eligible for marriage: {}",is_eligible);

    let num:u32 = 1232323323;

   let c= char::from_u32(num);

    if let Some(c) = char::from_u32(num){
        println!("Char is {}",c);
    }else if let None =  char::from_u32(num){
        println!("No char value");
    }


}




