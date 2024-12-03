fn main() {
    let F1 = || {
        // no input or output params or return type but just a statement
        println!("Hello World")
    };

    // Fn()

    F1();

    (|| {
        println!("Hello World-2");
    })(); // directly executing the closure

    (|a: i32, b: i32| {
        let c = a + b;
        println!("c:{}", c);
    })(100, 200); // executing the closure

    let c1 = (|a: i32, b: i32| -> i32 { return a + b })(10, 20);
    println!("c1:{}", c1);

    let F2 = |a: i32, b: i32| -> i32 { return a + b };

    let c2= F2(100,200);
    println!("c2:{}", c2);

    // let f3: &dyn Fn(i32,i32)->i32;
    // f3 = &F2;
    let c3=calc1(123,232,&|a:i32,b:i32|->i32{
        return a+b;
    });
    println!("c3:{}", c3);

    let c4 = calc1(10,20,&mul);
    println!("c4:{}", c4);

    let c5= calc2(10,20,|i:i32,j:i32|->i32{
        return i+j;
    });
    println!("c5:{}", c5);

}

fn calc1(a:i32,b:i32,f1: &dyn Fn(i32,i32)->i32)->i32{
    return f1(a,b);
}


fn calc2<F>(a:i32,b:i32,f:F)->i32 where F:Fn(i32,i32)->i32{
return f(a,b);
}


fn mul(a:i32,b:i32)->i32{
    return a *b;
}