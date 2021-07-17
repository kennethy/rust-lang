# 19. Advanced Features

## 19.1. Unsafe Rust

Underlying computer hardware is inherently unsafe, unsafe operations need to be allowed to perform certain tasks when directly interacting with the operating system.

`unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks. The intent is that the programmer will ensure the code inside the `unsafe` block will access memory in a valid way.

### Unsafe Superpowers

- Deference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of `union`S

### Deferencing a Raw Pointer

Unsafe rust has two new types called raw pointers, `*const T` (immutable) and `*mut T` (mutable). The asterisk isn't the deference operator and it's part of the type name.

How raw pointers are different from references and smart pointers:
- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to have the same location.
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic cleanup

```rust
let mut num = 5;


// we can declare raw pointers that are possibly invalid
// wouldn't have been allowed if we declare reference or mutable reference
// immutable/mutable raw pointers are allowed
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;


// raw pointers can only be dereferenced in unsafe block
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

### Calling an Unsafe Function or Method

```rust
// bodies of unsafe functions are already considered unsafe, so no need
// to declare another unsafe block
unsafe fn dangerous() {}

unsafe {
    // unsafe functions can only be invoked in unsafe block
    dangerous();
}
```

**Creating a Safe Abstraction over Unsafe Cover**

`slice::from_raw_parts_mut`  takes a raw pointer and a length, and it creates a slice.

```rust
use std::slice;

// function doesn't need to be marked as unsafe if it contains unsafe block
// because it creates only valid pointersfrom the data the function has access to
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), // unsafe because it must trust the offset is valid
        )
    }
}
```

**Using `extern` Functions to Call External Code**

Calls to other languages are considered unsafe since they don't enforce Rust's rules and guarantees.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### Accessing or Modifying a Mutable Static Variable

Global variables in Rust are also called static variables, but they could be problematic when accessed from different threads. They are declared in screaming snake case, and must be annotated with the `static` keyword.

```rust
static HELLO_WORLD: &str = "this is a static variable";

fn main() {
    println!("static variable is: {}", HELLO_WORLD);
}
```

Difference between constants and immutable static variables:
- static variable have a fixed address in memory, whereas constants are allowed to be duplicated their data whenever they are used.
- static variables can be mutable whereas constants are immutable. Mutating a static variable is an unsafe operation.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // any code that read/write mutable static variable need to be `unsafe` block
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

### Implementing an Unsafe Trait

A trait is unsafe when at least one of its method has some invariant that the compiler can't verify.

```rust
unsafe trait Foo {
    // ...
}

unsafe impl Foo for i32 {
    // ...
}
```

## 19.2. Advanced Traits

### Associated Types

Associated types seem similar to that of generics, but we use associated types to avoid annotating the types multiple times.

They connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

It allows us to define traits that use some types without needing to know exactly what those types are until the trait is implemented.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

### Default Generic Type Parameters and Operator Overloading

Operators listed in `std::ops` can be overloaded.

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

Add Trait with default generic type. `Self` is the type that implements the trait.

```rust
pub trait Add<Rhs=Self> { // <--- syntax is called default type parameters
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

If we want to support `Rhs` to be of some other type:

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

### Fully Qualified Syntax for Disambiguation

When a type implements multiple traits, and the traits all have a method that has a same name.

```rust

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // or Human::fly(&person);
}
```

To disambiguate associated functions (static methods on the trait, methods that do not use self):

```rust
fn main() {
    println!("{}", SomeType::foo());
    println!("{}", <SomeType as AnotherType>::foo());
}
```

### Using Supertraits To Require One Trait's Functionaity with Another Trait

```rust
use std::fmt;

trait ThisTrait: ThatTrait {
    // ...
}
```


### Use Newtype Pattern to implement External traits on External types

Due to the orphan rule, we are only allowed to implement a trait on a type as long as the trait or the type is local to our crate.

To come around this restriction, we can declare a struct (that is local to our crate) to wrap around the external type.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

## 19.3. Advanced Types

### Creating Type Synonyms with Type Aliases

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

### The Never Type that Never Returns

`!` is the `Never` type, and functions that return never are called *diverging functions*.

```rust
fn bar() -> ! {
    // ...
}
```

Previously, we were able to use `continue` even though match arms require all branches to return the same type.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

This worked because `continue` has a `!` value, and expressions of type `!` can be coerced into any other type.

`panic!` returns `!` which is why we were able to implement `unwrap()` on `Option<T>`.

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Never ending `loop` also returns `!`.

```rust
loop {
    println!("never ending);
}
```

### Dynamically Sized Types and the `Sized` Trait

Rust need to know certain details like how much space to allocate for a value of a particular type.

Dynamically Sized Types (also known as DSTs) or unsized types allow us to use values whose size can only be known at runtime.

`&str` represents two values, the address of the `str` and its length.

The golden rule of dynaically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

Every trait is a dynamically sized type when can refer to by using the name of the trait. We must put trait objects behind a pointer in order to use them, like `&dyn Trait`, or `Box<dyn SomeTrait>`.

Rust has a special trait `Sized` to determine whether a type's size is known at compile time. It is automatically implemented for everything whose size is known at compile time.

Rust implicitly adds a bound on `Sized` to every generic function.

```rust
fn generic<T>(t: T) {}
```

is equivalent to:

```rust
fn generic<T: Sized>(t: T) {}
```

Generic functions only work on types that have a known size at compile time, relax this restriction with:

```rust
fn generic<T: ?Sized>(t: &T) {}
```


`?Sized` means `T` may or may not be `Sized`, and since the `T` might not be, we need to put them behind a pointer, so we use `&T` instead.

## 19.4. Advanced Functions and Closures

### Function Pointers

Regular functions can passed to functions. Functions are typed with `fn`. The type implements all three of closure traits (`Fn`, `FnMut`, `FnOnce`), so it can be used for a function that expects a closure.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    // ...
}
```

```rust
let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

// or
let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
```

Enum values are be initialized 
```rust
enum Status {
    Value(u32), // this is actually a initializer function, Status::Value
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

### Returning Closures

Closures are represented by traits, so they can't be returned directly. Rust doesn't know how much space it needs to allocate for the closure.

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```