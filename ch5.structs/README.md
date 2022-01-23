# Chapter 5. Using Structs to Structure Related Data

## 5.1. Declaration and Instantiation

```rust
struct User {
    username: String,
    email: String,
    sign_in_acount: u64,
    active: bool,
}
```

### Initialization

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
```

### Creating Instances from Other Instances

```rust
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

### Tuple Structs

Tuple structs have the added meaning the struct name provides but don't have names associated with their fields; they just have the types of the fields.

Each struct you define is its own type.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32); // note Point is not the same as Color

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// access fields with dot notitation
println!("{}", black.0);
```

### Unit-Like Structs Without Any Fields

```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

### Ownership of Struct Data

References are allowed in a struct but it would requires the use of `lifetimes`.

## 5.2. Struct Examples

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

### Derived Traits

Using `{}` in `println!` for structs require them to implement `std::fmt::Display`.

Use `{:?}` for smaller structs and `{:#?}` for pretty-print.

```rust
#[derive(Debug)] // make `Rectangle` printable with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

println!("{:?}", ret); // print ret and its field values
println!("{:#?}", ret); // pretty prints ret
```

### `dbg!`

The method takes the ownership of the object we are passing into, display where the value as a result of the expression, where it expresses, and returns the ownership.

```rust
let rect = Rectangle {
    width: dbg!(30 * foo),
    height: 100
}

dbg!(&rect) // `borrow` since we don't want dbg! to own `rect`
```

## 5.3. Method Syntax

Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for.

`&self` is a shorthand for `self: &Self`.

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

### `->` opreator

Rust uses `automatic referencing and dereferencing` in place of the `->` operator. `&`, `&mut`, or `*` will automatically be added so the `object` will match the signature of the method (receiver and name of the method).

```rust
p1.distance(&p2);

// is equivalent to

(&p1).distance(&p2);
```

### Associated Functions

Associated functions that don't have `self` are allowed.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


// then call
Rectangle::square(50);
```

### Multiple `impl` Blocks

Multiple `impl` block is valid syntax.