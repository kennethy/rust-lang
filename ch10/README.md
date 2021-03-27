# Chapter 10. Generic Types, Traits, and Lifetime

## Generics

### Example with largest.

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
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

Add common functionalities to different types. Similar to interfaces. One One restriction is that we cannot implement external traits on external types.

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

impl<T: Display + PartialOrd> Pair<T> {
    // ...
}
```

### Blanket Implementations

When implementing a trait on any type that satisfies the trait bounds