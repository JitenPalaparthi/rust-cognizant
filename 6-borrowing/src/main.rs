fn main() {
    let s1: String = String::from("Hello World");
    {
        let s2 = &s1; // borrowing
        println!("s2:{}", s2);
    }

    println!("{}", s1);

    let mut s3: String = String::from("Hello");
    //let s4 = &s3; // This is immutable borrow
    {
        let s4: &mut String = &mut s3; // Mutable borrow .
                             // How long the ownership is temparorily with s4?
        s4.push_str("World");
    }
    s3.push_str("! Be stable and calm");
   // s4.push_str("! Be stable and calm");

    println!("s3:{}", s3);

    let s5 = &mut s3;

    s5.push_str(".Yes it should be");
    println!("s3:{}", s3);
   // s5.push_str(".Yes it should be");

    //println!("s3:{}", s4);
}

// borrowing: Temporarily moving the ownership to another variable
// 3.You can only have one mutable reference or multiple immutable references to a value at a time.
