fn main() {
    let e1: Employee = Employee::new(1001,"Jiten".to_string(),"Jitenp@outlook.com".to_string(),"9618558500".to_string());
    e1.display_employee();

    // let s1: &str = "Jiten";

    // let s2: String = "Jiten".to_string();
    let name = "Jiten";
    let email= "JItenp@outlook.com";
    let contact = "9618558500";

    let s1: Student = Student::new(&1001,name,email,contact,"active".to_string());
    s1.display_student();

}


struct Employee{
    id:i32,
    name:String,
    email:String,
    contact:String
}

impl Employee{
    fn new(id:i32,name:String,email:String,contact:String)->Self{
        Employee{id:id,name:name,email:email,contact:contact}
    }
    fn display_employee(&self){
        println!("Employee Details");
        println!("id: {}",self.id);
        println!("name: {}",self.name);
        println!("email: {}",self.email);
        println!("contact: {}",self.contact);
    }
}

struct Student<'a>{
    id:&'a i32,
    name:&'a str,
    email:&'a str,
    contact:&'a str,
    status:String
}

impl<'a> Student<'a>{
    fn new(id:&'a i32,name:&'a str,email:&'a str,contact:&'a str,status:String)->Self{
        Student{id:id,name:&name,email:&email,contact:&contact,status:status}
    }
    fn display_student(&self){
        println!("Student Details");
        println!("id: {}",self.id);
        println!("name: {}",self.name);
        println!("email: {}",self.email);
        println!("contact: {}",self.contact);
        println!("status: {}",self.status);
    }
}

// traits
// implementing traints
// impl and dyn 
// implementing multiple traits

// package management and library traits

// closures and threads

// smart pointers

// restful services (actix-web)

// dockerizing restful service 

// deploying the service in a kubernetes instance

// deploying a service as a lamda

// tokio async and await runtime