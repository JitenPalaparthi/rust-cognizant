fn main() {

    //let mut v1 = Vec::new(); // must tell the type

    let mut v1:Vec<i32>= Vec::new();

   // let mut v3 = Vec::with_capacity(capacity)

    let mut v2: Vec<i32> = vec![10,20,30,40]; // creating a vector using vector macro

    let mut v3: Vec<&str> = vec!["hello","world","how","are","you","doing","?"];

    // let mut arr1 = [1,2,3,4];
    // println!("{:?}",arr1);

    // for mut i in arr1{
    //     i = i * 10;
    //     println!("{}",i);
    // }
    // println!("{:?}",arr1);
    // arr1[0]=100;
    println!("length of v2:{} capsity:{}",v2.len(),v2.capacity());

    for i in &v2{
        println!("{}",i);
    }

    v2.push(60);
    v2.push(61);
    v2.push(63);
    v2.push(64);

    println!("length of v2:{} capsity:{}",v2.len(),v2.capacity());
    v2.push(65);
    println!("length of v2:{} capsity:{}",v2.len(),v2.capacity());

    for i in v2.iter(){
        println!("{}",i);
    }


    v2.remove(7);

    for i in v2.iter(){
        println!("{}",i);
    }

    v2.pop();
    for i in v2.iter(){
        println!("{}",i);
    }
let s1 = sum_of(&v2);

let s2 = sum_of_slice(v2.as_slice());

println!("s1:{}",s1);
println!("s2:{}",s2);



}


fn sum_of(vec :&Vec<i32>)->i32{
    let mut sum =0;
    for i in vec{
        sum+=*i;
    }
    sum
}

fn sum_of_slice(slice :&[i32])->i32{
    let mut sum =0;
    for i in slice{
        sum+=*i;
    }
    sum
}

// create a function to pass the vector as a input param
// return a vector as a return type with reverse the actual vector