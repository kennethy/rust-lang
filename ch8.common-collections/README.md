# Chapter 8. Collections

## 8.1. Vectors

The type we will be storing in the vector needs to annotated if it starts out empty.

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

## 8.2. Storing UTF-8 Encoded Text with Strings

The `String` type is a growable, mutable, owned, UTF-8 encoded string type. String slices are of `str` type, and it's usually seen in its borrowed form `&str`.

### Declaration
```rust
let mut s0 = String::new();

let s1 = String::from("hello world");

// or 
let s2 = "hello world".to_string();
```

### Update

A `String` can grow in size and its content can change.

```rust
let mut s = "hello ";

// push a string
s.push_str("world"); // push_str does not take ownership

// or push a char
s.push('a'); // push does not take ownership
```

### Concat

String's add operator (+) fn takes ownership.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3; // s1 has been moved and cannot be used after

// or
let s = format!("{}-{}-{}", s1, s2, s3); // does not take ownership
```

The compiler is able to coerce `&String` argument into a `&str` with the help of *deref coercion*, which turns `&s2` (of `String` type) into `&s2[..]`.

For more complicated string concatenation, use the `format!` macro. The macro also doesn't take ownership of any of its parameters.

### Iteration

Indexing on strings are not allowed as it would not make sense for Unicode scalar values. Each unit may take more than a byte.


Use `chars()` if you want to iterate on individual Unicode scalar values.

```rust
for c in “我喜欢你”.chars()
    println!("{}", c);
}

// or
for c in "hello world".bytes() {
    println!("{}", b);
}
```

**Internal Representation**

A `String` is a wrapper over a `Vec<u8>`.

## 8.3. Hash Maps

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.

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

### Hash Maps and Ownership

For types that implement `Copy` trait, like `i32`, the values are copied into the hash map.

For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

```rust
use std::collections::HashMap;

let a = String::from("A");
let b = String::from("B");

let mut map = HashMap::new();
map.insert(a, b)
// a and b will no longer be valid since they are moved
```

### Read

Use `get()` to read the value associated to the key. It returns an `Option<&V>`. We don't want `get()` to take ownership of the key, so a reference of the key is passed.

```rust
let value = map.get(&key);
```

We can also print the entries in the map.

```rust
for (key, value) in &m {
    println!("key: {} val: {}", key, value);
}
```

### Update

**Overwriting a value by using identical keys**

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

**Insert if the key does not already exist**

`entry()` returns a mutable reference (`&mut V`) that allows us to update the entry in the hash map.

```rust
let mut scores = HashMap::new();
scores.entry(String::from("key")).or_insert(100);
```

**Update based on old value**

```rust
let text = "hello world wonder world";

let mut m = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### Hashing Functions

By default, `HashMap` uses the `SipHash` hashing function.

To switch to another hashing function, a hasher that implements the `BuildHasher` trait needs to be specified.