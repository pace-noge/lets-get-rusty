enum Message {
    Quit,
    Move: { x: i32, y: i32},
    ChangeColor: {i32, i32, i32},
}

impl Message {
    fn move(x: &i32, y: &i32) -> Message {
        Message::Move{x, y}
    }
}

fn main() {
    
}