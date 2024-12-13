//pub async fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>>
//pub fn read<P>(path: P) -> io::Result<Vec<u8>>

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let r1=add1(10,20);
    let f1 = add2(10,20);
    let r2=f1.await; // waiting for the result from a future

    let a = 100;
    let b= 200;

    let f2 = |a1:i32,b1:i32| async move{
        a1+b1
    };

    let r3= f2(100,200).await;

    let f3 = async{
        println!("do something");
        a+b
    };

    let f4 = async{
        println!("do something");
        a+b
    };

   let(r4,r5)= tokio::join!(f3,f4);
   println!("r1:{} r2:{} r3:{} r4:{}:r5:{}",r1,r2,r3,r4,r5);

}

fn add1(a:i32,b:i32)->i32{
    a+b
}

async fn add2(a:i32,b:i32)->i32{
     a+b
}
// tokio -> it contains a async runtime and also libraries