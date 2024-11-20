fn main() {
let mut name: &str = "Congnizant"; // allocated on stack
name = "Congnizant India Pvt LTD"; // oi reallocats and gives the pointer to str 
let companay: String = "Congnizant".to_string(); // allocated on heap

let mut company_name: String = String::from("Cognizant India"); // allocated on heap
 company_name.push_str(" Private Limited");


 let n1 = "Conginizant";
 let c1 = "India Private Limited";
 let n2:String = String::from("India Private Limited");
 let f1 = n1.to_owned() + c1; // convert n1 to String and add the &str reference to that string
 // + operator has been implemented for String 
let f2 = n2 + c1;

let s1 = f2.as_str(); // as_str is a method on f2 object

let mut f3: String = String::new(); // this is a function from String module

// String::from("Hello");


// f3 = f2 + &n2; // The problem is not with the + operator but the problem is borrowing 

f3.push_str(n1);

f3.push_str(c1);

println!("f1:{}\nf2:{}\nf3:{}",f1,f2,f3);

}


// + arthimetic operations
// on numbers 
// + Add 