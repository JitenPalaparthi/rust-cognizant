fn main() {
    // unconditional loop

    let mut counter = 1; // init
    loop {
        if counter > 10 {
            // condition
            break;
        }
        println!("{}", counter);
        counter += 1; // post condition
    }

    counter = 1;

    loop {
        if counter > 10 {
            break;
        }
        if counter % 2 == 0 {
            println!("even number:{}", counter);
        }
        counter += 1;
    }

    counter = 1;

    loop {
        if counter > 10 {
            break;
        }
        if counter % 2 == 0 {
            counter += 1;
            continue;
        }
        println!("odd number:{}", counter);
        counter += 1;
    }

    let mut i = 1;

    'outer: loop {
        // labelled block
        if i > 5 {
            break;
        }
        let mut j = 10;
        loop {
            if j < 5 {
                break 'outer;
            }
            println!("i:{} j:{}", i, j);
            j -= 1;
        }
        i += 1;
    }

    println!("loop can also be used as an expression");
    let mut counter = 1;
    let mut sum = 0;
    let result = loop {
        if counter > 10 {
            break (sum, counter); // when you break the loop what has to be the result;
        }
        sum += counter;
        counter += 1;
    };
    println!("sum:{} counter:{}", result.0, result.1);

    let mut counter = 1;
    let mut sum = 0;
    let (s1, c1) = loop {
        if counter > 10 {
            break (sum, counter); // when you break the loop what has to be the result;
        }
        sum += counter;
        counter += 1;
    };
    println!("sum:{} counter:{}", s1, c1);
}
