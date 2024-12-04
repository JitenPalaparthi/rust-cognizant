use core::time;
use std::thread;
fn main() {
    
    let handle1 = thread::spawn(||{
        for i in 1..10{
            println!("executing in a separate thread:{}",i);
            thread::sleep(time::Duration::from_millis(100));
            if i==5{
                panic!("just panicing the thread-1 purposefully");
            }
        }
    });

    let handle2= thread::spawn(print_numbers);

    for i in 1..10{
        println!("executing in a main thread:{}",i);
        thread::sleep(time::Duration::from_millis(200));
    }

 let r1=  handle1.join();

match r1{
    Ok(_)=>println!("Spawn Thread-1 executed successfully"),
    Err(e)=>println!("Error executing the thread-1:{:?}",e),
}

match handle2.join(){
    Ok(_)=>println!("Spawn Thread-2 executed successfully"),
    Err(e)=>println!("Error executing the thread-2:{:?}",e),
    //Box<dyn Any + Send>
}
   println!("main is going to exit");
}

fn print_numbers(){
    for i in 1..10{
        println!("executing in a separate thread from a function:{}",i);
        thread::sleep(time::Duration::from_millis(100));
    }
}

// err has some other values, check how to handle it..

