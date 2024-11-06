use std::ops::Index;

fn main() {
    let a=100;
    let b= 200;

    {
        let mut c =300;
        let str1= String::from("Hello World");
        let str2 ="Hello World";
        let box1 = Box::new(100);
    }
    /*
    str1 --> 0x110013 (stack)
    ptr -> 0x1010ff (on heap)
    len -> 11
    cap -> 11
     */

    let mut box2 = Box::new(12312.123);

    let mut box3 = &mut *box2; // transfer? or copy?
    //let mut box3 = &mut box2; 
    // box2 is dropped here
    //*box2=100.123;

     /*
    box2 --> 0x110013 (stack)
    ptr -> 0x1010ff (on heap)
    len -> 4
     */
    *box3 = 12312.123;
    println!("box2:{}",*box2);

    {
        let mut ok1 =false;
        println!("address of ok1={:p}",&ok1);

        let char1 = 'A';
    }

    let ok1: bool=true;
    println!("address of ok1={:p}",&ok1);
    let ok1: bool = true;
    println!("address of ok1={:p}",&ok1);



    let mut v1: Vec<i32> = vec![1,2,3,4,5];


    for mut v in  &mut v1{
        *v = *v *2;
        println!("v={}",v);
    }

    println!("v={:?}",v1);

    let mut index=0;
    loop{
        if index>=v1.len(){
            break;
        }
        println!("{}",v1.index(index));
        index+=1;
    }

    let arr1 = [10,20,30];

    let mut index=0;
    loop{
        if index>=arr1.len(){
            break;
        }
        println!("{}",arr1[index]);
        index+=1;
    }


}

// the borrow of box
// you are borrowing the pointer itslef but not the heap allocated data