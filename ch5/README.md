# Chapter 5. Using Structs to Structure Related Data

# Declaration

```rust
struct User {
    username: String,
    email: String,
    sign_in_acount: u64,
    active: bool,
}
```

# Initialization

```rust
let user1 = User {
    email: String::from("test@test.com"),
    username: String::from("kenneth"),
    active: true,
    sign_in_count: 1
};

// shortcut
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 0
    }
}

// copy with rest, and mutable user
let mut user2 = {
    email: String::from("test@test.com"),
    username: "user2",
    // remaining fields not explicitly set 
    // will have the same value as user1
    ..user1
}

user2.email = String::from("updated@test.com"); // allowed because `mut`
```

# Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32); // note Point is not the same as Color

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// access fields with dot notitation
println!("{}", black.0);
```

# Derived Traits

```rust
#[derive(Debug)] // make `Rectangle` printablewith {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

println!("{:?}", ret); // print ret and its field values
println!("{:#?}", ret); // pretty prints ret
```

# Defining Methods

```rust
impl Rectangle {
    // just want to borrow, so use &self
    // self param is used for transformation and
    // don't want to return previous instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

# Associated Functions

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

then call
```rust
Rectangle::square(50);
```
