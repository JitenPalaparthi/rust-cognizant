use std::ptr::addr_of;

static mut LENGTH: i32 = 0;

fn main() {
    let s1 = "Hello world";

    //  let l1 = str_length(s1);
   // {
        let mut l1: i32 = 0;

        let result = str_length1(s1, &mut l1);

        println!("length of s1:{}", *result);
   // }

    let s2: String = String::from("Hello Cognizant minds!");

    let l2 = str_length2(&s2);
    println!("length of s2:{}", *l2);

    let result2 = str_length3(&s2);

    println!("length of s2:{}", *result2);
}

// fn str_length4<'a>(s:&'a str)->&i32{
//    let l = s.len() as i32; // l is created inside a function
//    return &l; // dangling reference
//    //addr_of!(LENGTH);
// }

fn str_length1<'a>(s: &str, l: &'a mut i32) -> &'a i32 {
    *l = s.len() as i32; // l is created inside a function
    return l; // dangling reference
}

fn get_value<'a>(l: &'a mut i32) -> &'a i32 {
    *l = 30;
    return l;
}

fn str_length2<'a>(s: &'a str) -> Box<i32> {
    let l = s.len() as i32;
    let b = Box::new(l);
    return b;
}

// func strLength(str string)(*int){
//     l:= new(int)
//     l = len(str)
//     return l
// }

fn str_length3<'a>(s: &'a str) -> &'static i32 {
    unsafe {
        LENGTH = s.len() as i32; // l is created inside a function
        return &LENGTH; // dangling reference
    }
    // addr_of!(LENGTH)
}
