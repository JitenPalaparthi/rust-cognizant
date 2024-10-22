fn main() {
    let mut a: i32 = 100;
    let mut b = a; // copy
    println!("a:{} b:{}", a, b);
    a = 200;
    println!("a:{} b:{}", a, b);

    {
        let mut a1: i32 = 100;
        let mut b1 = a1; // copy
        println!("a:{} b:{}", a1, b1);
        a1 = 200;
    }
    //println!("a:{} b:{}", a1, b1);

    let s1: String = String::from("Hello World"); // String is stored in heap memory
    let s2 = s1; // ownership is transffered to s2
                 //println!("s2:{} s1:{}", s2,s1);
    println!("s2:{}", s2);
    // let l = get_length(s2);
    //println!("s2:{} length:{}",s2,l)
    let (l1, s2) = get_length_ownershio(s2);

    println!("length:{} s1:{}", l1, s2);
}

fn get_length(str1: String) -> usize {
    str1.len()
}

fn get_length_ownershio(str1: String) -> (usize, String) {
    let l1 = str1.len();
    //let s1 = str1;
    return (l1, str1);
}
// rust implements trait(s)
// copy trait
// drop trait

// a = 100
// b = 100

// Rules of Ownership

// 1. Each and every value in rust has a single owner
// 2. When the owner goes out of scope the value is dropped , and its memory is deallocated
// 3. We can have only one mutable reference or multiple immutable references to a value at a time
