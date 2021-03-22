enum IpAddrKind {
    V4(i32, i32, i32, i32),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anon struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(value) => println!("value {}", value),
            _ => println!("something else"),
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("use state {:?}", state);
            25
        },
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("matched coin {}", value_in_cents(Coin::Penny));
    println!("matched coin {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}
