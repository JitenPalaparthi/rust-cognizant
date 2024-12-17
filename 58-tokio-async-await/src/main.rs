use futures::future;
// use futures::{join};
use tokio::sync::mpsc;
#[tokio::main]
async fn main() {

    // let mut num = 0;

    // let f1 = async{
    //     for i in 1..100{
    //         tokio::time::sleep(tokio::time::Duration::from_millis(500));
    //         num+=1;
    //     }
    // };

    // let f2 = async{
    //     for i in 1..100{
    //         tokio::time::sleep(tokio::time::Duration::from_millis(500));
    //         num-=1;
    //     }
    // };
       //join!(f1,f2);
       let (tx,mut rx)= mpsc::channel::<i32>(1);
        let f1 = async move{
            for i in 1..=3{
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                tx.send(i).await.unwrap();
            }
            println!("finished sending values");
            return;
           // drop(tx);
           //tx.closed().await;
        };

      let f2= async{
        while let result=rx.recv().await{
            match result {
                Some(value)=>println!("Value Received:{}",value),
                None=> {
                    println!("receiver is not receiving vlaues");
                    return 
                }
            }
        }
       };

       tokio::join!(f1,f2); // executor
       //let f3=future::join(f1, f2);
      // f3.await;
      // join!(f1, f2);
      println!("channel is closed")
       //f1.await;
      // f2.await;

}

// correct the commented code  by enabling arc with mutex;
