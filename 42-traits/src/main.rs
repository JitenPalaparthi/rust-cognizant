fn main() {
    accept_all_types1(100);
    accept_all_types1(true);
    accept_all_types1(123.123);
    accept_all_types1("Hello World");
    accept_all_types1("Hello World".to_string());
    accept_all_types1(Box::new(100));

    accept_all_types2(&100);
    accept_all_types2(&true);
    accept_all_types2(&123.123);
    accept_all_types2(&"Hello World");
    accept_all_types2(&"Hello World".to_string());
    accept_all_types2(&Box::new(100));

    accept_all_types3(100,"Hello World")
}


trait Universal{} // does not have definitions

impl<T> Universal for T{}

fn accept_all_types1<T:Universal>(data:T){
    println!("Accept value type:{}",std::any::type_name::<T>());
}
 
fn accept_all_types2(data: &dyn Universal){
    println!("Accept value type:{}",std::any::type_name::<&dyn Universal>());
}

// monomorphization
fn accept_all_types3<T:Universal,U:Universal>(a:T,b:U){
    println!("Accept value type:{}",std::any::type_name::<T>());
}

// fn add<T:Universal>(a:T,b:T)->T{

// }

// create a universal trait 
// add(a:T,b:T)->T
// this add function should work for all numbers
// size,usize, u8,u16,u32,u64,i8,i16,i32,i64,f32 and f64