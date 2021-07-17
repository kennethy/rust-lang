# Chapter 17. Object Oriented Features of Rust

## 17.1 Characteristics of Object Oriented Languages

**Objects Contain Data and Behaviour**

Rust is object oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums.

**Encapsulation that hides Implementation Details**

Control encapuslation with `pub` keyword.

**Inheritance as a Type System and As Code Sharing**

Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide.

## 17.2 Using Traits to Allow Values of Different Types

**Defining a Trait for Common Behaviour**

We can't add data to traits and they are used for abstraction across common behaviour.

Suppose we want to write a GUI library, and users may wish to support drawing new types. We use traits to accomplish this:

```rust
pub trait Draw {
    fn draw(&self);
}
```

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

The definition aboves works differently from a generic type parameter, as it allows us to have values with multiple types. Generic type parameter can only be substituted with one concrete type at a time.


**Implementing the Trait**

```rust
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw
    }
}
```

**Trait Objects Perform Dynamic Dispatch**

`monomorphization` is when the compiler generates the nongeneric implementation of functions and methods for each concrete type. The result of it is doing `static dispatch`, which means the compiler knows what method you are calling at compile time.

`dynamic dispatch` is used when using trait objects. The compiler doesn't know all the types that might be used so Rust uses the pointers inside the trait objects to determine which method to call (runtime cost).

**Object Safety Is Required for Trait Objects**

A trait is object safe if all the methods defined in the trait have the follwoing properties:
1. the return type isn't `Self`.
2. There are no generic type parameters.


An example of a trait whose methods are not object safe

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

## 17.3. Implementing State Pattern

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // calls as_ref() on the Option because we want the ref as opposed to ownership
        // calls unwrap() since we know there will always be a Some value

        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // .take() takes the value in Some() out and replace with None
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}
```

```rust
trait State {

    // use Box<Self> because it means the method is valid only when called on a Box holding the type
    // this syntax takes ownership of Box<Self>, and will invalidate old state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn aprove(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
```

```rust
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

### 19.5. Macros

`macro` refers to a family features in Rust: declarative macros with `macro_rules!` and three kinds of procedural macros:

1. Custom `#[derive]` macros that is used on structs and enums, and specifies the new code added
2. Attribute-like macros that define custom attributes usable on any item
3. Function-like macros that look like function calls but operate on the tokens specified as their arguments.

### Difference betwen Macros and Functions

Macros are a way of writing code that writes other code, which is also known as *metaprogramming*.

Macros are take variable amount of variables. It can implement a trait on a given type. Functions can't because it gets called at runtime and a trait needs to be implemented at compile time.

### Delarative macros with `macro_rules!`

Delarative macros match patterns and generate code based on captured expressions.

- `#[macro_export]` indicates the macro will be made available whenever the crate in which the macro is defined is bought into the scope.

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

### Procedural Macros for Generating Code from Attributes


Procedural macros accept some code as an input. operate on the that code, and produce some code as an output.

Their definitions must reside in their own crate in a special crate type.

```rust
use proc_macro; // brings in `TokenStream`

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // ...
}
```


### How to Write a Custom `derive` Macro

Usage

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;


#[derive(HelloMacro)]
struct PanCakes;

fn main() {
    Pancakes::hello_macro();
}
```

Trait Creation

`cargo new hello_macro --lib`

```rust
// src/lib.rs
pub trait HelloMacro {
    fn hello_macro();
}
```

Derivable Trait

`cargo new hello_macro_derive --lib` within the directory of the `hello_macro` crate.

Then we add the following to the `Cargo.toml` for `hello_macro_derive`.

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```
