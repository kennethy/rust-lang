# Chapter 4. Understanding Ownership

**Ownership Rules**
1. Each value in Rust has a variable that's called *owner.*
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped (via Drop).

**Move**

```rust
let y1 = String::from("str");
let y2 = y1; // underlying data is moved from y1 to y2
println!("{}", y1); // this will not be allowed
```

**Clone**

```rust
let x1 = String::from("str");
let x2 = x1.clone(); // performs deep copy
println!("x1 = {}, x2 = {}", x1, x2);
```

**Copy**

Simple scalar values and nothing that requires allocation can implement `Copy`, to prevent the `move` behaviour.

**Ownership Transfer**

```rust
let s = String::from("hello");
takes_ownership(s); // `move`: ownership transferred to scope of fn

let x = 5;
makes_copy(5); // `copy`: is called since x i32 is copy

let s1 = gives_ownershop(); // whatever is returned from fn
let s2 = String::from("hello");
let s3 = takes_and_gives_back(s2); // s2 is moved
```