fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // ANCHOR: here
    impl Message {
        fn call(&self) {
            // o corpo do método seria definido aqui
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    // ANCHOR_END: here
}
