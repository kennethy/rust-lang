# Chapter 10. Generic Types, Traits, and Lifetime

## Generics

### Example with largest.

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for &i in list { // &i here because &[T] is passed
        if i > largest {
            largest = i;
        }
    }

    largest
}
```

### Example with Struct.

```rust
struct Point<T> {
    x: T,
    y: T,
}

// or multiple types
struct Point<T, U> {
    x: T,
    y: U,
}
```

### Example with Enum

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Method Definitions

```rust
impl<T> Point<T> {
    fn x(&self) -> T {
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

### Monomorphization

The process of turning generic code into specific code by filling in the concrete types that are used when compiled. As a result, using generics would not induce runtime cost.

## Traits

Add common functionalities to different types. Similar to interfaces. One restriction is that we cannot implement external traits on external types.

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

// then we can call
let p = Point { x: 1, y: 2 };
println!("{}", p.summarize());
```

### Default implementation

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

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", );
}

// or long form with trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### Specifying Multiple Traits
```rust
pub fn notify(item: &(impl Summary + Display)) {

// or
pub fn notify<T: Summary + Display>(item: &T) {
```

### Specifying Traits with Where clause

```rust
pub fn notify<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// or
pub fn notify<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
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

### Conditionally implement methods

```rust
struct Pair<T> { ... }

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // ...
}
```

### Blanket Implementations

When implementing a trait on any type that satisfies the trait bounds

## Lifetimes

Lifetime annotations describe the relationships of the lifetime of multiple references to each other without affecting the lifetime.

Annotations to help the borrower checker to ensure there are not dangling pointers.
### Syntax

```rust
&i32
&'a i32
&'a mut i32
```

### Annotations in Functions

Used when one of the parameters is a reference.

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.

Lifetime syntax is about connecting the lifetime of various parameters and return values of functions — to prevent dangling references.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
```

### Annotations in Structs

Used when a struct uses references. 

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### Lifetime Ellison

Compiler will add the lifetime annotations automatically based on three rules below. The compiler will complain if any of the parameters don't get its lifetime resolved. Rules apply to to function definitions and `impl` blocks.

1. Each parameter that is a reference gets its own lifetime annotation.
2. If there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If one of the input lifetime parameters is `&self`, or `&mut self`, then the lifetime of `self` is assigned to all output lifetime parameters.

### Annotations in Method Defintions

Lifetime names for struct fields always need to be declared.

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