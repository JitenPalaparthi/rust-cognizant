fn main() {
    // move
    let mut n1 = 100;
    incr1_normal(n1);
    println!("n1:{}", n1);

    let mut incr1_closure = move || {
        n1 += 1;
    };
    incr1_closure();
    println!("n1:{}", n1);

    // mutable borrow
    incr2_normal(&mut n1);
    println!("n1:{}", n1);

    let mut incr2_closure = || {
        // automatically mutable borrow &mut T
        n1 += 1;
    };
    incr2_closure();
    println!("n1:{}", n1);

    // normal borrow

    let normal_borrow = || {
        // normal &T
        println!("n1:{}", n1);
    };
    normal_borrow();

    let fn1 = multiplyer1(10);
    let r1 = fn1(12);
    println!("r1:{}", r1);

    let boxfn1= multiplyer2(13);
    let r2=boxfn1(14);
    println!("r2:{}", r2);
}

fn incr1_normal(mut num: i32) {
    // equallent to FnOnce , move
    num += 1;
}

fn incr2_normal(num: &mut i32) {
    // equallent to FnMut
    *num += 1;
}

fn multiplyer1(m: i32) -> impl FnOnce(i32) -> i32  {
    return  move |x: i32| -> i32 {
        return x * m;
    };
}

fn multiplyer2(m: i32) -> Box<dyn FnOnce(i32) -> i32> {
    return  Box::new( move |x: i32| -> i32 {
        return x * m;
    });
}

// fn multiplier<T>(factor: i32) -> impl T
// where
//     T: Fn(i32) -> i32, // Specify the trait bound
// {
//      return move |x|{ x * factor};
// }


fn sq(mut n:i32)->Box<i32>{
    n = n *n;
    return Box::new(n);
}

// fn sq2(mut n:i32)->&i32{ // dangling
//     let n1 = n *n; // creating a variable inside a function
//     //n = n *n;
//     return &n1 // return the reference out side of the function
// }