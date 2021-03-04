enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> () {
        match self {
            Message::Quit => {
                println!("quit");
            }
            Message::Move { x, y } => {
                println!("move to {},{}", x, y);
            }
            Message::Write(message) => {
                println!("{}", message);
            }
            Message::ChangeColor(r, g, b) => {
                println!("change color to {},{},{}", r, g, b);
            }
        }
    }
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 10, y: 10 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(12, 23, 45);
    q.call();
    m.call();
    w.call();
    c.call();

    let some_number = Some(10);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;
    let sum = 5 + some_number.unwrap_or_default();
    let sum2 = 5 + absent_number.unwrap_or_default();
    println!("5+some(10).unwrap_or_default() = {}", sum);
    println!("5+None.unwrap_or_default() = {}", sum2)
}
