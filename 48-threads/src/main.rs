use core::time;
use std::thread;
fn main() {
    
    let handle1 = thread::spawn(||{
        for i in 1..10{
            println!("executing in a separate thread:{}",i);
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    let handle2= thread::spawn(print_numbers);

    for i in 1..10{
        println!("executing in a main thread:{}",i);
        thread::sleep(time::Duration::from_millis(200));
    }

   handle1.join().unwrap();
   handle2.join().unwrap();
   println!("main is going to exit");
}

fn print_numbers(){
    for i in 1..10{
        println!("executing in a separate thread from a function:{}",i);
        thread::sleep(time::Duration::from_millis(100));
    }
}