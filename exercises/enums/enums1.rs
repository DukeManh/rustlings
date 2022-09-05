// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor
}

struct DOB (
    u32,
    u32,
    u32
);

fn main() {
    let my_date_of_birth: DOB = DOB(2001, 07, 03);

    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
    println!("I was born in {}", my_date_of_birth.0);
}
