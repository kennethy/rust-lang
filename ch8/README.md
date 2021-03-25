# Chapter 8. Collections

## Vectors

Declaration
```rust
let v1: Vec<i32> = Vec::new();

// or
let v2 = vec![1, 2, 3];
```

### Vector update

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```

### Reading Vecs

```rust
let v = vec![1, 2, 3, 4];

// 1st way with indexing
let third: &i32 = &v[2];
println!("{}", third); // => 3

// 2nd way with get()
match v.get(2) {
    Some(third) => println!("{}", third),
    None => println!("invalid"),
}
```

### Immutable/Mutable References

The following won't compile because there can't be both an immutable and mutable reference in the same scope.
The reason is that upon modifying the original object, its references may have moved upon resizing.
```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6); // fist may no longer be valid because contents of v may be moved else where

println!("The first element is: {}", first);
```

### Iterating a Vec

```rust
let v = vec![1, 2, 3];

for i in &v {
    println!("{}", v);
}
```

### Modifying the Vec while iterating

```rust
let mut v = vec![1, 2, 3];

for i in &v {
    *i += 10;
}
```

### Using Enum

Enum variants are of the same enum type, so variants under the same enum can be placed in a vector.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```