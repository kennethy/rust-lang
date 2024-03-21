# Chapter 10. Generic Types, Traits, and Lifetime

## 10.1. Generic Data Types

### In Function Definitions

When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what the name means.

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

### In Struct Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

// declaration
let point = Point { x: 10, y: 10 }
```

Or with multiple types:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

// declaration
let a = Point { x: 10, y: 10.0 } // allowed
let b = Point { x: 10, y: 10 } // not allowed since we specified T and U to be different
```

### In Enum Definitions

```rust
enum Option<T> {
    Some(T),
    None,
}
```
or

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### In Method Definitions

Specifying `<T>` right after `impl` to let Rust know the type in the angle brackets in `Point` is a generic type rather than a concrete type.

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// or specific to one type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

It's possible the generic types are different from `impl` and method signatures.
```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// usage
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };
let p3 = p1.mixup(p2);
```

### Monomorphization

The process of turning generic code into specific code by filling in the concrete types that are used when compiled. As a result, using generics would not induce runtime cost.

## 10.2. Traits: Defining Shared Behaviour

The purpose of `traits` is to add common functionalities to different types. Similar to interfaces. One restriction is that we cannot implement external traits on external types.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("({}, {})"), self.x, self.y);
    }
}
```

Then we can call

```rust
let p = Point { x: 1, y: 2 };
println!("{}", p.summarize());
```

### Default implementation

A default implementation can be defined in the trait block. Default implementations can also call other methods in the same trait, even if those methods don't have a default implementation.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    // default implementation
    // can call other methods in the same trait
    fn summary(&self) -> String {
        format!("(Read more {})", self.summarize_author());
    }
}
```
### Traits as Parameters

We can use traits to define functions that accept many different types.

Instead of using a concrete type for the `item` parameter, we allow `item` to be any type that implements the `Summary` trait.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", );
}
```

Alternatively, we can use the long form with `trait bound`:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

It's needed when a function allows different params of the same type and we require all of them to implement the  same trait:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

### Specifying Multiple Traits
```rust
pub fn notify(item: &(impl Summary + Display)) {}

// or
pub fn notify<T: Summary + Display>(item: &T) {}
```

### Specifying Traits with Where clause

Using too many trait bounds make the functions hard to read. The `where` clause comes to the rescue.

```rust
pub fn notify<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// or
pub fn notify<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

### Return types that implements traits

```rust
fn returns_summarizable() -> impl Summary {
```

Only allowed if the fn returns a single type. The following is not allowed.

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { ... }
    } else {
        Tweet { ... }
    }
}
```

### Fixing the `largest` function with trait bounds

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

### Conditionally implement methods

We can use trait bounds with an `impl` block to implement methods conditionally.

```rust
struct Pair<T> { ... }

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            // ...
        } else {
            // ...
        }
    }
}
```

### Blanket Implementations

We can also conditionally implement a trait for any type that implements for another trait. Implementations of a trait on any type that satisfies the trait bounds are called `blanket implementations`.

```rust
impl<T: Display> ToString for T {
    // ...
}
```

## 10.3. Lifetimes

Lifetime annotations describe the relationships of the lifetime of multiple references to each other without affecting the lifetime.


### Preventing Dangling References with Lifetimes

The main purpose of lifetimes is to prevent dangling references.

The following code would not compile because `x` doesn't "live long enough". It goes out of scope when the inner scope ends but `r` is still valid for the outer scope.

```rust
{
    let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

### The borrower checker

At compile time, Rust compares the size of the two lifetimes and see that `r` has a lifetime of `'a` but that it refers to a memory with a lifetime of `'b`. The program fails to compile because `'b` is shoter than `'a`: the subject of the reference doesn't live as long as the reference.

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
} 
```

The following is fine since `'b` lives longer than `'a`.

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}    
```

### Generic Lifetimes in Functions

The following won't compile because the return type needs a generic lifetime parameter to tell whether the reference being returned refers to `x` or `y`.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Syntax

```rust
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

### Lifetime Annotations in Functions

Used when one of the parameters is a reference.

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.

Lifetime syntax is about connecting the lifetime of various parameters and return values of functions — to prevent dangling references.

The signature marks that: for some lifetime `'a`, the lifetime of the returned reference will be valid as long as both the parameters are.

When we pass concrete references to `longest`, the concrete lifetime that is substited for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
```

We as humans can see `string1` will remain valid, but the rust compiler would not be able to.

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

### Thinking in Terms of Lifetimes

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.

### Annotations in Structs

The following annotation means an instance of `ImportantExcerpt` can't outlive the reference it holds in its `part` field.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### Lifetime Elision

Compiler will add the lifetime annotations automatically based on three rules below. The compiler will complain if any of the parameters don't get its lifetime resolved. Rules apply to function definitions and `impl` blocks.

1. Each parameter that is a reference gets its own lifetime annotation.
2. If there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If one of the input lifetime parameters is `&self`, or `&mut self`, then the lifetime of `self` is assigned to all output lifetime parameters.

### Annotations in Method Defintions

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name, because those lifetimes are part of the struct's type (how we defined above).

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

### Static lifetime

`static` means the reference can live for the entire duration of the program. All string literals have the static lifetime.

```
let s: &'static str = "I have a static lifetime.";
```

### Generic type parameters, trait bounds, and lifetimes together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```