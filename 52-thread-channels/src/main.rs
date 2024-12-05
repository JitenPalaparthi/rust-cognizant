use core::time;
use std::{sync::mpsc, thread};

fn main() {
   let (tx1,rx1)= mpsc::channel();

   let tx2 = tx1.clone(); // new instance.. 
   
   let handler1= thread::spawn(move||{
    for i in 1..10{
        let str1 = format!("sending values from publisher-1 -->{}",i);
        thread::sleep(time::Duration::from_millis(500));
        tx1.send(str1).unwrap();
    }

   });

   let handler2= thread::spawn(move||{

    for i in 10..=20{
        let str2 = format!("sending values from publisher-2 -->{}",i);
        thread::sleep(time::Duration::from_millis(500));
        tx2.send(str2).unwrap();
    }

   });

   let handler3= thread::spawn(move||{
        for rec in rx1{
            println!("receiving-->{}",rec);
        }
   });
   handler1.join().unwrap();
   handler2.join().unwrap();
   handler3.join().unwrap();
   println!("exiting main")
}
