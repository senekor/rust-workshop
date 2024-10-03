//

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 12, y: 20 },
        Message::Write("Hello, world!".into()),
        Message::ChangeColor(120, 50, 200),
    ];

    println!("{:#?}", messages);
}
