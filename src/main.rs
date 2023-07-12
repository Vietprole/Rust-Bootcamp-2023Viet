fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
    enum MessageOne {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl MessageOne {
        fn show_message(msg: MessageOne) {
            //println!("{}", msg);
            match msg {
                MessageOne::Quit => println!("Quit"),
                MessageOne::Move { x, y } => println!("Move to ({},{})", x, y),
                MessageOne::Write(string) => println!("{}", string),
                MessageOne::ChangeColor(a, b, c) => println!("Change color to {}, {}, {}", a, b, c )
            }
        }
    }
    
    
    fn exercise1() {
        let msgs:[MessageOne; 3] = [
            MessageOne::Quit,
            MessageOne::Move { x: 1, y: 3 },
            MessageOne::ChangeColor(255, 255, 0),
        ];
    
        for msg in msgs {
            MessageOne::show_message(msg);
        }
    }
    exercise1();
}
