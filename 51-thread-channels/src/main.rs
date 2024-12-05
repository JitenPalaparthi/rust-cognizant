use core::time;
use std::{sync::mpsc, thread};
fn main() {
   let (tx,rx)= mpsc::channel::<String>();

   let handler1 = thread::spawn(move ||{
    for i in 1..10{
        thread::sleep(time::Duration::from_millis(500));
        let str1 = format!("Publisher-1-->{}",i);
       let result= tx.send(str1);
       match result{
        Ok(_)=> println!("successfully sent data to the channel"),
        Err(e)=>println!("There seems to be an error in sending data{:?}",e),
       }
    }
   });

   let handler2= thread::spawn(move ||{
    for receive in rx{
        println!("Receiving values from the sender---> {}",receive)
    }
   });

   handler1.join().unwrap();
   handler2.join().unwrap();
   println!("finished sending values to the channel, exiting main")

}

// csp -> comminicating sequential processess
// do not communicate by sharing memory; share memory by comminicating
// channels
// mpsc -> multiple producer and single consumer