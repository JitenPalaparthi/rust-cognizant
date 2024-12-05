use core::time;
use std::{sync::mpsc::channel, thread};

fn main() {
    let (tx1,rx1)=channel();
    let (tx2,rx2)=channel();
  //let tx2 = tx1.clone();
  //let rx2= channel();
 

let sender1 = thread::spawn(move||{
    for i in 1..=10{
        thread::sleep(time::Duration::from_millis(500));
        tx1.send(i).unwrap();
    }
});

let receive1 = thread::spawn(move||{
    for rec in rx1{
        println!("receiving from sender-1-->{}",rec);
        tx2.send(rec*rec).unwrap();
    }

});
let receive2 = thread::spawn(move ||{
    for rec in rx2{
        println!("receiving from sender-2-->{}",rec);
    }
});

sender1.join().unwrap();
receive1.join().unwrap();
receive2.join().unwrap();

}

// using channel is not sharing the data rather passing the data through the conduit/queue