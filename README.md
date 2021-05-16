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

`dynamic dispatch` is used when using trait objects. The compiler doesn't know all the types that might be used so Rust uses teh pointers inside the trait objects to determine which method to call (runtime cost).

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