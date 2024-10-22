fn main() {
    let s1: String = String::from("Hello World");
    let l1 = calc_length(&s1);

    println!("length: {} s1: {}", l1, s1);
    let s2 = "Hello world,how are you doing!";

    let l2 = calc_length(s2);
    println!("length: {} s2: {}", l2, s2);

    let mut a = 100;

    // let b = &mut a;

    // //let c = a; // copy

    // *b= 200;

    // println!("b:{}",*b);

    let b = a; // copy
    let c = a; // copy
    let d = a; // copy
    {
        let e: &i32 = &a; // immutable references
        let f = &a; // immutable references
        println!("e:{} f:{}", e, f);
    }
    {
        let g = &mut a;
        *g = 300;
        println!("g:{}", *g);
    }

    let b = &mut a;

    //let c =  a; // copy

    *b = 200;

    println!("b:{}", *b);
    //println!("e:{} f:{}",e,f);
}

fn calc_length(s: &str) -> usize {
    return s.len();
}

// 1.Each value in Rust has a single owner.
// 2.When the owner goes out of scope, the value is dropped, and its memory is freed.
// 3.You can only have one mutable reference or multiple immutable references to a value at a time.
