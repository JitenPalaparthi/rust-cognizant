fn main() {
    
    let mut num = 100;
    let mut num2 = 200;

    let print= ||{ //Fn
        println!("num={}",num);
    };

    let mut incr1 = ||{  //FnMut
        //num+=num2;
        num+=1;
    };

    incr1();
    incr1();

    println!("num={}",num);


    let mut incr2 = |mut i:&mut i32|{ 
        //num+=num2;
        *i+=1;
    };

    incr2(&mut num);

    println!("num={}",&num);



    let mut incr3= move ||{ // FnOnce
        num2 = num2+1;
        println!("inside of the closure num2:{}",num2);
    };
    incr3();
    println!("outside of the closure num2:{}",num2);
    //  print_type(&incr1);
    //  print_type(&incr2);
    //  print_type(&incr3);

}

// by reference &T

// fn print_type<T>(_:&T){
//     println!("{}",std::any::type_name::<T>());
// }

