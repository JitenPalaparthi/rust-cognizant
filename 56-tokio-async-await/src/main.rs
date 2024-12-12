#[tokio::main]
async fn main() {
    tokio::select! {
        _= do_some_async_stuff1()=> {
            println!("do_some_async_stuff1() completed first")
        }
        _= do_some_async_stuff2()=> {
            println!("do_some_async_stuff2() completed first")
        }
    }

    let num =100.12;
    let f=async move {
        println!("Hello World");
        Ok::<f64,()>(num)
    };

    let r: Result<f64, ()> = f.await;
    
    match r{
        Ok(k)=>println!("returned :{}",k),
        Err(())=>println!("there is no error"),
    }

} 

async fn do_some_async_stuff1(){
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!("do_some_async_stuff is finished executing");
}

async fn do_some_async_stuff2(){
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!("do_some_async_stuff is finished executing");
}

