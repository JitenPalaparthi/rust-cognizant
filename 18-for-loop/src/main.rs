fn main() {
    println!("1-5 but not 5");
  for i in 1..5{
    println!("{}",i);
  }
println!("1-5 including 5");
  for i in 1..=5{
    println!("{}",i);
  }

  println!("1-100 even numbers");

  for num in 1..=100{
    if num%2==1{
        continue;
    }
    print!("{} ",num);
  }


  println!("iterate chars in a string");
  let str1=String::from("Hello World!");

  for ch in str1.chars(){
        print!("{} ",ch);
  }
  println!("iterate chars in a string");
  //let str1 = "Hello ，世界";
  for bt in "Hello, 世界!".bytes(){
    print!("{}={} ",bt,bt as char);
  }
  println!("iterate chars in a string");
  for ch in "Hello, 世界!".chars(){
    print!("{}",ch);
}
println!();

let str1 = "Hello World!";

}

// use only loop as an expression.