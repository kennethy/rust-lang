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


