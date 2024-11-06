fn main() {
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", arr1);

    let slice1 = &arr1[..]; // all the elements in the arr1 are referenced to the slice1
    println!("slice1:{:?}", slice1);
    // let str1: &str = "hello world";

    let slice2 = &arr1[1..4];
    println!("slice2:{:?}", slice2); // elements from 1 to 4 but not 4
    let slice3 = &arr1[1..=4];
    println!("slice3:{:?}", slice3); // elements from 1 to 4 including 4th element

    let slice4 = &arr1[4..];
    println!("slice4:{:?}", slice4); // elements from 4 to the end
    let slice5 = &arr1[..6];
    println!("slice5:{:?}", slice5); // elements from 0th to 6th but not 6th

    {
        let slice6 = &arr1; // technically this is not a slice+
        println!("slice6:{:?}", slice6);
    }
    println!("arr1:{:?}", arr1);

    let s1 = String::from("Hello World!");

    let h1 = &s1[..5];
    let w1 = &s1[6..];

    println!("h1:{}", h1);
    println!("w1:{}", w1);

    let arr2 = [10, 40, 20, 50, 60];

    let sum1 = sumOf(arr2);

    println!("sum of arr2:{}", sum1);

    let sum2 = sumOfs(&arr1);

    let sum3= sumOfs(slice1);

    let sum4 = sumOfs(slice2);


    let mut arr3 = [1,2,3,4,5,6];

    let mut slice1 = &mut arr3[..];
    println!("slice1:{:?}",slice1);
    double_slice(slice1);
    println!("slice1:{:?}",&mut slice1);

}

fn sumOf(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }
    sum
}

fn sumOfs(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }
    sum
}


fn double_slice(slice:&mut [i32]){
    for i in slice{
        *i = *i + *i;
    }
}

// create a program to take string as a argument and return the length of the string 
// dont use built in function of strins write code
// the function must acces both &str and String