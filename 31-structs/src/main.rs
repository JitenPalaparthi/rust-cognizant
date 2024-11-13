use core::fmt;

fn main() {
    {
    let u1 = User {
        id: 101,
        name: "Jiten".to_string(),
        email: "jitenp@outlook.com".to_string(),
        contact: "9618558500".to_string(),
        status: true,
    };

    println!("user u1 --> {:?}",u1);
    println!("user u1 --> {:#?}",u1);
    println!("user u1 --> {}",u1);
    println!("-------------->>>>>>");
    let mut u3 = Box::new(u1);
    u3.contact="12312".to_string();
    u3.id=101;
    println!("-------------->>>>>>");
}

println!("Hello world, checing how stuff works");
let u2 = User {
    id: 102,
    name: String::from("Priya"),
    email: "priya@outlook.com".to_string(),
    contact: "9813432345".to_string(),
    status: true,
};

println!("user u2 --> {:?}",u2);
println!("user u2 --> {:#?}",u2);
println!("user u2 --> {}",u2);
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
    contact: String,
    status: bool,
}

impl fmt::Display for User{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              write!(f, "User id:{}, name:{}, email:{}, contact:{}, status:{})", self.id, self.name,self.email,self.contact,self.status)
    }
}

impl Drop for User{
    fn drop(&mut self) {
        println!("Dropping User!{}",self);
    }
}