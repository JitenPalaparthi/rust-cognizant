fn main() {
    let d1 = divide1(10.12, 12.34);
    println!("d1:{}", d1);
    let d2 = divide1(10.34, 0.0);
    println!("d2:{}", d2);
    let d3 = divide1(0.0, 123.123);
    println!("d3:{}", d3);

    let d4: Result<f64, String> = divide2(10.12, 12.34);
    match d4 {
        Ok(x) => {
            println!("divid is {:.2}", x)
        }
        Err(s) => {
            println!("Error:{} ", s)
        }
    }
    let d4: Result<f64, String> = divide2(10.12, 0.0);
    match d4 {
        Ok(x) => {
            println!("divid is {:.2}", x)
        }
        Err(s) => {
            println!("Error:{} ", s)
        }
    }

    let d4: Result<f64, String> = divide2(10.12, 0.0);
    println!("Using if else instead of match case");
    if let Ok(x) = d4 {
        println!("divid is {:.2}", x)
    } else if let Err(s) = d4 {
        println!("Error:{} ", s)
    }

    let d4 = divide2(10.12, 1.2).expect("as value does not come, it panics");
    println!("divide:{:.2}", d4);
    // the above code panics
}

fn divide1(a: f64, b: f64) -> f64 {
    a / b
}

fn divide2(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("divided by zero error"))
    } else {
        Ok(a / b)
    }
}
