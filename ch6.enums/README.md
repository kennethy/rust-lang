# Chapter 6. Enums and Pattern Matching

## 6.1. Enum Definition

```rust
enum IpAddrKind {
    V4,
    V6,
}

// can be declared like the following:
let four = IpAddKind::V4;
let six = IpAddKind::V6;
```

### Enum Values

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

`IpAddr::V4()` and `IpAddr::V6()` constructors are available as a result of defining the enum.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    v6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
```

An enum that has a variety of types embedded in its variants.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anon struct
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### Enum Methods

We are able to define methods on enums using `impl`.

```rust
impl Message {
    fn call(&self) {
        // do something
    }
}

let m = Message:Write(String::from("hello"))l
```

### `Option<T>` enum

The concept of a value being present or absent is represented by the `Option<T>` enum. It requires programmers to convert an `Option<T>` to `T` before operating on `T`.

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let absent_number: Option<i32> = None;
```

## 6.2. The `match` control flow

Th power of `match` comes from the expresiveness of the patterns and the fact the compiler confirms that all possible cases are handled.

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
        Coin::Quarter => 25,
        _ => (), // '_' matches any value, `()` unit value, does not do anything
    }
}
```

### Patterns that Bind to Values

```rust
#[derive(Debug)]
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
```

The pattern will be passed in the value associated to the enum value.

```rust
match coin {
    // --snip--
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    }
}
```

### Matching with `Option<T>`

```rust
match x {
    None => None,
    Some(i) => {
        // do somethign with i
    }
}
```

### Matches Are Exhaustive

All possible patterns must be covered or else the program would not compile.

We can use the `_` placeholder if we won't be using rest of the patterns.

```rust
let dice_roll = 9;

match dice_roll {
    3 => foo(),
    7 => bar(),
    _ => baz(), // _ placeholder if the rest of the values won't be used
}
```

Or we can pass it at the end.
```rust
match dice_roll {
    3 => foo(),
    7 => bar(),
    other => baz(other),
}
```

Or we can use `()` for an no-op.

```rust
match dice_roll {
    3 => foo(),
    7 => bar(),
    _ => (),
}
```

## 6.3. Concise Control Flow with `if_let`

The `if let` syntax is a less verbose way to match one pattern while ignoring the rest. Though we lose the exhaustive checking that `match` enforces.

```rust

let config_max = Some(3u8);

// before
match config_max {
    Some(max) => println!(max),
    _ => (),
}

// after
if let Some(max) = config_max { // max binds to the value inside the `Some`
    println!(max);
}
```

We can also use the `else` clause.

```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("Quarter from {:?}", state);
} else {
    count += 1;
}
```