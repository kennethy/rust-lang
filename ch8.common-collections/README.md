# Chapter 8. Collections

## 8.1. Vectors

The type we will be storing in the vector needs to annotated if it starst out empty.

Like any other `struct`, a vector is freed when it goes out of scope. All of its contents are also dropped.

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

Access with `[]` and `&` which returns a reference.

```rust
let v = vec![1, 2, 3, 4];

let third: &i32 = &v[2];
println!("{}", third); // => 3
```

Access with `get()` which returns an `Option<&T>`.

```rust
match v.get(2) {
    Some(third) => println!("{}", third),
    None => println!("invalid"),
}
```

### Immutable/Mutable References

The following won't compile because there can't be both an immutable and mutable reference in the same scope.

The reason is that upon modifying the original object, its references may have been moved upon resizing.

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6); // first may no longer be valid because contents of v may be moved else where

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

for i in &mut v {
    *i += 10;
}
```

### Using Enum to Store Multiple Types

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

## Strings

### Declaration
```rust
let s1 = String::from("hello world");

// or 
let s2 = "hello world".to_string()
```

### Update
```rust
let mut s = "hello ";

// push a string
s.push_str("world");

// or push a char
s.push('a');
```

### Concat

String's add (+ operator) fn takes ownership.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// or
let s = format!("{}-{}-{}", s1, s2, s3); // does not take ownership
```

### Iteration

Indexing on strings are not allowed as it would not make sense for Unicode scalar values. Each unit may take more than a byte.

```rust
for c in “我喜欢你”.chars() {
    println!("{}", c);
}

// or
for c in "hello world".bytes() {
    println!("{}", b);
}
```

## HashMap

### Declaration

All keys must be the same type, and all values must be the same type.
```rust
use std::collections::HashMap;

let mut m = HashMap::new();

m.insert(String::from("one"), 1);
m.insert(String::from("two"), 2);
```

or 

```rust
let v1 = vec![1, 2, 3];
let v2 = vec![String::from("one"), String::from("two"), String::from("three")];

// key/value type is inferred
// zip() returns tuples
// collect() turns tuples into a HashMap
let m: HashMap<_, _> = v1.into_iter().zip(v2.into_iter()).collect();
```

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point
```

### Update

Overwrite
```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

Insert if the key does not already exist
```rust
let mut scores = HashMap::new();
scores.entry(String::from("key")).or_insert(100);
```

Update based on old value
```rust
let mut scores = HashMap::new();
scores.insert(String::from("key"), 123);

let entry = score.entry(String::from("key")).or_insert(0);
*entry += 1;
```

### Iteration

```rust
for (key, value) in &m {
    println!("key: {} val: {}", key, value);
}
```