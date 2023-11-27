# Chapter 4. Understanding Ownership

## 4.1. What is Ownership?

In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

### The Stack and the Heap

Stacks follow a last in, first out order. All data stored on the stack must have a known, fixed size.

Data with unknown size is stored on the heap. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data. The location is always at the top of the stack.

**Ownership Rules**
1. Each value in Rust has a variable that's called *owner.*
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped (via Drop).

### Variable Scope

A variable is no longer valid when it is out of scope.

```rust
{ // s is not valid since it hasn't been declared yet
    let s = "hello";
    // do stuff with s
} // this scope is now over, and s is no longer valid.
```

### The `String` Type

String literals have a fixed size and will be put onto the stack. Use the `String::from(...)` to allocate the string on the heap.

### Memory and Allocation

In Rust, the memory is automatically returned once the variable that owns it goes out of scope.

The special  `drop` funtion is called automatically, which would deallocate the memory.

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

Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. Therefore, stack-only data will not be invalidated.

Implement the `Copy` trait to ensure a variable is still valid after assignment to another variable. This is allowed only on variables that hasn't implemented the `Drop` trait. In general, nothing that reuqires allocation or is some form of resource can implement `Copy`.

**Copy**

Simple scalar values and nothing that requires allocation can implement `Copy`, to prevent the `move` behaviour.

```rust
let x = 5;
let y = x; // no ownership transfer, because x, i32, is stored on stack

println!("x = {}, y = {}", x, y);
```

### Ownership and Functions

Passing a variable to a function will move or copy, just as assignment does.

```rust
let s = String::from("hello");
takes_ownership(s); // `move`: ownership transferred to scope of fn, s is no longer valid here

let x = 5;
makes_copy(5); // `copy`: is called since x i32 is copy
```

### Return Values and Scope

Returning values can also transfer ownership.

```rust
let s1 = gives_ownership(); // whatever is returned from fn
let s2 = String::from("hello");
let s3 = takes_and_gives_back(s2); // s2 is moved
```

**References**

References are used so that values can be used without transferring ownership.

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

## 4.2. References and Borrowing

Borrowing is when a reference is passed as a parameter to a function. We are not allowed to modify the value the reference points to.

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);

// s1 remains valid because it is 'borrowed'.
```

The opposite of referencing is by dereferencing, using the `*` operator.

**Mutable References**

Variables are immutable by default, so are references. Mutable references allow you to modify the data it references to.

```rust
let mut s = String::from("hello world");
fn modify(&mut s); // &mut s to allow fn to update value s points to

fn modify(s: &mut String) {
    s.push_str(", appended content");
}
```

Only one mutable reference to a particular piece of data in a particular scope. 

```rust
let mut s = String::from("hello world");

{
    let r1 = &mut s;
    // r1 goes out of scope, so making a new reference is allowed.
}

let r2 = &mut s;
```

In addition, we cannot have a mutable reference while we have an immutable one in the same scope. This mechanism is used in Rust to prevent data races.

A data race happens when:
- Multiple pointers access the same data at the same time.
- At least one of them is used to write to the data.
- No mechanism to synchronize access to the data. 

```rust
let mut s = String::from("hello");

let r1 = &s; // ok
let r2 = &s; // ok
let r3 = &mut s; // not ok
```

Users of an immutable reference would not expect the value to suddently change. Therefore, multiple immutable references are allowed but not when there's an immutable one and a mutable one.

The scope of reference start from where it's first being initialized and continues through the last time the reference is used.

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;

println!("{}, {}", r1, r2); // r1 r2 out of scope after here

let r3 = &mut s; // ok here
println!("{}", r3);
```

### Dangling References

Rust compiler guarantees the data will not go out of scope before the reference to the data does.

### Rule of References

- At any given time, you can have either one mutable reference or any number of immutable references. This is to prevent data races at compile time.
- References must always be valid.

## 4.3. The Slice Type

A slice lets you reference a contiguous sequence of elements in a collection rather than the whole collection. It's a reference so it does not have ownership.

### String Slices

In an example in which an index is derived from iterating a string, the index could become invalid or meaningless if the string is later mutated.

The solution to this problem is string slices.

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

**String Literals Are Slices**

`s` is of type `&str`, and it's an immutable reference.

```rust
let s = "Hello world!";
```

**String Slices as Parameters**

When a function takes in a `&str`, we can pass a string slice directly, or a slice of the `String` or a reference to the `String`. This flexibility is provided by `deref coercions`.

### Other Slices

```rust
let a = [1, 2, 3, 4];
let slice = &a[1..3]; // type &[i32]
assert_eq!(slice, &[2, 3]);
```
