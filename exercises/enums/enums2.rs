// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.
#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
    Echo(String),
    Quit(char),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn send(message: &Either<&Message>) {
    match message{
        Either::Nothing => {
            println!("Not a valid message");
        },
        Either::Some(m) => {
            println!("Sending messaage: {:?}", m);
        }
    };
}

enum Either<T> {
    Nothing,
    Some(T)
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit('0'),
    ];

    for message in &messages {
        message.call();
    }

    let message = Either::Some(&messages[0]);
    let not_message = Either::Nothing;
    send(&message);
    send(&not_message);
}