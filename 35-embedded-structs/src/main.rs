use std::ops::Add;

fn main() {
    let addr1 = Address::new(
        "doorno 1-43".to_string(),
        "Hyderabad".to_string(),
        "India".to_string(),
        "501232".to_string(),
    );
    let person1 = Person::new(
        101,
        "Jiten".to_string(),
        "jitenp@outlook.com".to_string(),
        "9618558500".to_string(),
        addr1,("Facebook.com/jpalaparthi".to_string(),"linkedin.com/jpalaparthi".to_string(),"twitter.com/jpalaparthi".to_string())
    );
    //person1.display();
    //person1.get_social();

    println!("{:#?}",person1);
    person1.display();
}

#[derive(Debug)]
struct Address {
    line1: String,
    city: String,
    country: String,
    pincode: String,
}

impl Address {
    fn new(line1: String, city: String, country: String, pincode: String) -> Self {
        Self {
            line1: line1,
            city: city,
            country: country,
            pincode: pincode,
        }
    }
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    email: String,
    contact: String,
    address: Address,
    social:(String,String,String),
}

impl Person {
    fn new(id: i32, name: String, email: String, contact: String, addr: Address,social:(String,String,String)) -> Self {
        Self {
            id: id,
            name: name,
            email: email,
            contact: contact,
            address: addr,
            social:social
        }
    }

    fn display(&self) {
        println!(
            "[id:{}\nname:{},email:{},contact:{},address:{:?},social:{:?}]",
            self.id, self.name, self.email, self.contact, self.address,self.social
        );
    }

    fn get_social(&self){
        //println!("socialmedia-1:{} socialmedia-2:{} socialmedia-3:{}",self.social.0,self.social.1,self.social.2);
        println!("social:{:?}",self.social);
    }
}
