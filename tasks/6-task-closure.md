Create a function 
fn calc(a:i32,b:i32,f1: &dyn Fn(i32,i32)->i32)->i32{
    return f1(a,b);
}

1- directly pass add function

2- create a subtract closure assign to a variable and pass to calc 

3- create a normal function, that satisfies the signature of the closure and pass it as a parameter (implement multiply)

The calc function should  be called thrice 1.to add two numbers 2. to subtract two numbers 3. to multiply two numbers.

