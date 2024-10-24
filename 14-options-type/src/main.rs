fn main() {
   let mut num1:Option<i32> = None;
   println!("num1, get_default:{}",get_default(num1));
   num1 = Some(100);
   println!("num1, get_default:{}",get_default(num1));
   
   let mut num1:Option<i32> = None;
   println!("num1, get_default:{}",get_or(num1,200));
   num1 = Some(100);
   println!("num1, get_default:{}",get_or(num1,200));

  // let num1:Option<i32>= None;

  // let n1 = num1.unwrap(); // unwrap returns the value only if there is Some, if there is None, it panics


  let num2: Option<i32> = Some(101);

    match num2{
        Some(v) if v%2==0=>{
            println!("{} is some and also even number",v);
        },
        Some(v) if v%2 !=0=>{
            println!("{} is some and also odd number",v);
        },
        None =>{
            println!("It is None, so cant say even or odd.There is nothing.")
        },
        _=>{}
    }

}

fn get_default(opt: Option<i32>) -> i32 {
    match opt {
        Some(v) => return v,
        None => return 0,
    }
}
fn get_or(opt: Option<i32>,df:i32) -> i32 {
    match opt {
        Some(v) => return v,
        None => return df,
    }
}
fn some_number(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n)
    } else {
        None
    }
}
fn get_default_if(opt:Option<i32>)->i32{
    if let Some(v)=opt{
        return v;
    }
    return 0;
}
