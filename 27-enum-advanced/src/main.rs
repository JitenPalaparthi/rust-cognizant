fn main() {
let m1 = Message::ChangeColour(10, 20, 30);
process_message(m1);
process_message(Message::Quit);
process_message(Message::Write("Hello Folks".to_string()));
process_message(Message::Move { x: 100, y: 200 });
}

enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColour(i32,i32,i32)
}

fn process_message(msg:Message){
    match msg{
        Message::Quit=>println!("just quiting"),
        Message::Move { x, y }=>print!("Moving to {} and {}",x,y),
        Message::Write(s1)=>{
            println!("Write the message: {}",s1);
        },
        Message::ChangeColour(r,g,b , )=>{
            println!("R:{} G:{} B:{}",r,g,b);
        },
    }
}

// fn process_message2(msg:Option<Message>){
//     match msg{
//         // Message::Quit=>println!("just quiting"),
//         // Message::Move { x, y }=>print!("Moving to {} and {}",x,y),
//         // Message::Write(s1)=>{
//         //     println!("Write the message: {}",s1);
//         // },
//         // Message::ChangeColour(r,g,b , )=>{
//         //     println!("R:{} G:{} B:{}",r,g,b);
//         // },
//     }
// }