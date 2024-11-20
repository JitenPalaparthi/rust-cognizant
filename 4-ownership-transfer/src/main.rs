fn main() {

    let mut a1 = 100;

    let b1 = a1 ; // copy  
    // 100 owned by a1 
    // another 100 ownwed by b1

    a1 = 200;

    let c1 = a1 + b1;

    println!("a1: {} b1:{} c1:{}",a1,b1,c1);

    let s1  = String::from ("Hello World");
    let s2 = s1; // the owner of "Hello World" is s2 but not s1 , hence s1 is no longer be the owner of the data
  // the ownership is transferred from s1 to s2
    println!("s2: {}", s2);

}

// There is only one owner for the data at any point time.
// If the data is moved (for heap) , then the ownership is transferred
// The old owner is no longer valid 

