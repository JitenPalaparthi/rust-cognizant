use std::{sync::{Arc, Mutex}, thread};

fn main() {
   //let mut shared_counter = 100;
   let num=10;
   let shared_counter = Arc::new(Mutex::new(num));
  
   let counter_thread1 = Arc::clone(&shared_counter);
   let handler1 = thread::spawn(move ||{
    for i in 1..=10{
      let mut counter = counter_thread1.lock().unwrap();
      *counter+=1;
      println!("current-thread1 counter:{}",*counter);
    }
}
);
let counter_thread2 = Arc::clone(&shared_counter);
    let handler2 = thread::spawn(move ||{
        for i in 1..=10{
            let mut counter = counter_thread2.lock().unwrap();
      *counter-=1;
      println!("current-thread2 counter:{}",*counter);
        }
   });


   handler1.join().unwrap();
   handler2.join().unwrap();
println!("Shared counter:{}",*shared_counter.lock().unwrap());

}
