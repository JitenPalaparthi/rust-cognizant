use std::thread;

use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
   let future1= add(10,20);
   let v = future1.await;
   println!("future value v:{}",v);

   let task1= tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "Hello World-1"
   });
   let task2= tokio::spawn(async {
    sleep(Duration::from_secs(1)).await;
    "Hello World-2"
});

   //tokio::join!(task1);

   /*
   
   tokio::task::spawn
    pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
    */

   /*std::thread
   pub fn spawn<F, T>(f: F) -> JoinHandle<T>
   where
       F: FnOnce() -> T,
       F: Send + 'static,
       T: Send + 'static,*/
  println!("{:?}",chrono::Local::now());
  sleep(Duration::from_secs(1)).await;
  let result1= task1.await.unwrap();
  let result2: &str= task2.await.unwrap();
  println!("Result1:{}",result1);
  println!("Result2:{}",result2);
  println!("{:?}",chrono::Local::now());

}

async fn add(i:i32,j:i32)->i32{
        i+j
}
