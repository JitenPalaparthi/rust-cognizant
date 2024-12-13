#[tokio::main]
async fn main() {

    let mut num = 0;

    let f1 = async{
        for i in 1..100{
            tokio::time::sleep(tokio::time::Duration::from_millis(500));
            num+=1;
        }
    };

    let f2 = async{
        for i in 1..100{
            tokio::time::sleep(tokio::time::Duration::from_millis(500));
            num-=1;
        }
    };
        tokio::join!(f1,f2);
}

// correct this program by enabling arc with mutex;
