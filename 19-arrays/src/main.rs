fn main() {
    let mut numbers: [i32; 5] = [1, 5, 7, 19, 12];

    println!("iterate an array");

    for i in numbers {
        print!("{} ", i);
    }

    println!("\nSum of elements in an array");

    let mut sum = 0;
    for i in numbers {
        sum += i;
    }

    println!("sum of numbers:{}", sum);
    numbers[0]=100;
    println!();
    for i in &mut numbers{
        *i = *i * *i;
        print!("{} ", i);
    }

    // let mut i =0 ;
    // for n in numbers{
    //     numbers[i]= n*n;
    //     print!("{} ", n*n);
    //     i+=1;
    // }

    println!("\n numbers:{:?}",numbers);

}

// what is an array
// it is a fixed size collection
// the size of the array should be compiler aware
// arrays are stored in stack memory
