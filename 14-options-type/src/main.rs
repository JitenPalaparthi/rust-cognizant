fn main() {
   let mut num1:Option<i32> = None;
   println!("num1, get_default:{}",get_default(num1));
   num1 = Some(100);
   println!("num1, get_default:{}",get_default(num1));
   
   let mut num1:Option<i32> = None;
   println!("num1, get_default:{}",get_or(num1,200));
   num1 = Some(100);
   println!("num1, get_default:{}",get_or(num1,200));


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
