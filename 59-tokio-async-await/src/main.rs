use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
       let (tx,mut rx)= mpsc::channel::<i32>(1);
        let f1 = tokio::spawn(async move{
            for i in 1..=3{
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                tx.send(i).await.unwrap();
            }
            println!("finished sending values");
            return;
        });

      let f2= tokio::spawn(async move{
        while let result=rx.recv().await{
            match result {
                Some(value)=>println!("Value Received:{}",value),
                None=> {
                    println!("receiver is not receiving vlaues");
                    return 
                }
            }
        }
       });
       f1.await.unwrap();
       f2.await.unwrap();
}
// correct the commented code  by enabling arc with mutex;
