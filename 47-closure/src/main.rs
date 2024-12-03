fn main() {

    // move 
    let mut n1 =100;
    incr1_normal(n1);
    println!("n1:{}",n1);

    let mut incr1_closure =move ||{
        n1+=1;
    };
    incr1_closure();
    println!("n1:{}",n1);

    // mutable borrow
    incr2_normal(&mut n1);
    println!("n1:{}",n1);

    let mut incr2_closure = ||{ // automatically mutable borrow &mut T
        n1+=1;
    };
    incr2_closure();
    println!("n1:{}",n1);

    // normal borrow

    let normal_borrow=||{ // normal &T
        println!("n1:{}",n1);
    };
    normal_borrow();
    
}


fn incr1_normal(mut num:i32){  // equallent to FnOnce , move
    num+=1;
}


fn incr2_normal(num:&mut i32){  // equallent to FnMut 
    *num+=1;
}
