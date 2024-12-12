use core::time;

use tokio::time::{sleep, Duration};
#[tokio::main] // this tells that the main function is using tokio
async fn main() {
    println!("Hello world");
    let f1 = async_await_func1;
    let f2 = async_await_func2;
    let f3 = || async {
        println!("async await closure started.. started");
        sleep(Duration::from_millis(500)).await;
        println!("async await closure started/.. finished");
    };

//    let  future1= f3();
//    let  future2= f1();
//    let  future3= f2();

    tokio::join!(f1(),f2(),f3()); // it is waited here until all async functions finish off their executiuon
    // sleep(Duration::from_millis(500)).await;
    // future1.await;
    // future2.await;
    // future3.await;
    println!("main finished execution");
}

async fn async_await_func1() {
    println!("calling async_await_func1.. started");
    sleep(Duration::from_millis(500)).await;
    println!("calling async_await_func1.. finished")
}

async fn async_await_func2() {
    println!("calling async_await_func2.. started");
    sleep(Duration::from_millis(200)).await;
    println!("calling async_await_func2.. finished")
}
