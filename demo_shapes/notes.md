crates.io

excercise

    lib.rs
        -> lib_greet()
    a.rs ->a_greet()
    b.rs ->b_greet()
    c.rs ->c_greet()
        a
            a1.rs ->a1_greet()
            a2.rs
        b 
            b1.rs
        c
            c1.rs
            c1
                c11.rs ->c11_greet()

        pub fn lib_greet(){
            println!("I am lib_greet")
            a_greet()
            b_greet()
            c_creet()
            a1_greet()
        }