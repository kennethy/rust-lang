# Chapter 6. Enums and Pattern Matching

# Declaration

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddKind::V4;
let six = IpAddKind::V6;
```

# Associated Enums

```rust
// with struct
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// a more compact way
enum IpAddr {
    V4(String),
    v6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
```

Each enum variant can have different types and amounts of associated data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    v6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anon struct
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

# Enum Methods

```rust
impl Message {
    fn call(&self) {
        // do something
    }
}

let m = Message:Write(String::from("hello"))l
```

# Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
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
```