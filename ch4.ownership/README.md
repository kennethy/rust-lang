# Chapter 4. Understanding Ownership

## 4.1. What is Ownership?

In Rust, memory is managed through a system of ownership with a set rule rules that the compiler checks.

## The Stack and the Heap

Stacks follow a last in, first out order. All data stored on the stack must have a known, fixed size.

Data with unknown size is stored on the heap. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data. The location is always at the top of the stack.

**Ownership Rules**
1. Each value in Rust has a variable that's called *owner.*
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped (via Drop).

**Move**

```rust
let y1 = String::from("str");
let y2 = y1; // underlying data is 'moved' from y1 to y2. Not calling it shallow copy because y1 is dropped.
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

```rust
let x = 5;
let y = x; // no ownership transfer, because x, i32, is stored on stack

println!("x = {}, y = {}", x, y);
```

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

**References**

```rust
let s1 = String::from("hello world");
let len = calculate_length(&s1);

// ...
// s is a reference to a String
// references allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize { // 
    s.len()
} // s goes out of scope but nothing happens
```

Borrowing is when a reference is passed as a parameter to a function. We are not allowed to modify the value the reference points to.

**Mutable References**

Only one mutable reference to a particular piece of data in a particular scope.

```rust
let mut s = String::from("hello world");
fn(&mut s); // &s to allow modify fn to update value s points to

fn(s: &mut String) {
    s.push_str(", appended content");
}
```

In addition, we cannot have a mutable reference while we have an immutable one in the same scope. 

```rust
 let mut s = String::from("hello");

let r1 = &s; // ok
let r2 = &s; // ok
let r3 = &mut s; // not ok
```

The scope of reference start from where it's first being initialized and continues through the last time the reference is used.

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;

println!("{}, {}", r1, r2); // r1 r2 out of scope after here

let r3 = &mut s; // ok here
println!("{}", r3);
```

**Rule of References**

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

## Slices

**String slices**

```rust
let s = String::from("hello world!");
let hello = &s[0..5]; // => "hello"
let world = &s[6..11]; // => world
```

**Subslices**

```rust
let a = [1, 2, 3];
let len = a.len();

// omit start
let slice = [..len];

// omit end
let slice = [0..];

// whole slice
let slice = a[..];  // same as a[0..len]
```
