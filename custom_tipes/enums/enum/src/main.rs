use std::fmt;

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

impl fmt::Display for WebEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebEvent::PageLoad =>
                write!(f, "Page loaded"),
            WebEvent::PageUnload =>
                write!(f, "Page unloaded"),
            WebEvent::KeyPress(c) =>
                write!(f, "Key pressed: '{}'", c),
            WebEvent::Paste(s) =>
                write!(f, "Pasted: \"{}\"", s),
            WebEvent::Click { x, y } =>
                write!(f, "Click at ({}, {})", x, y),
        }
    }
}

fn inspect(event: WebEvent) {
    println!("{}", event);
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    let x  = Operations::Add;
    let x2 = Operations::Subtract;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("{}", x.run(10, 5));  // 15
    println!("{}", x2.run(10, 5)); // 5
}
