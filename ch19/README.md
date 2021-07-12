# 19. Advanced Features

## 19.1. Unsafe Rust

Underlying computer hardware is inherently unsafe, unsafe operations need to be allowed to perform certain tasks when directly interacting with the operating system.

`unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks. The intent is that the programmer will ensure the code inside the `unsafe` block will access memory in a valid way.

### Unsafe Superpowers

- Deference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of `union`S

### Deferencing a Raw Pointer

Unsafe rust has two new types called raw pointers, `*const T` (immutable) and `*mut T` (mutable). The asterisk isn't the deference operator and it's part of the type name.

How raw pointers are different from references and smart pointers:
- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to have the same location.
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic cleanup

```rust
let mut num = 5;


// we can declare raw pointers that are possibly invalid
// wouldn't have been allowed if we declare reference or mutable reference
// immutable/mutable raw pointers are allowed
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;


// raw pointers can only be dereferenced in unsafe block
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

### Calling an Unsafe Function or Method

```rust
// bodies of unsafe functions are already considered unsafe, so no need
// to declare another unsafe block
unsafe fn dangerous() {}

unsafe {
    // unsafe functions can only be invoked in unsafe block
    dangerous();
}
```

**Creating a Safe Abstraction over Unsafe Cover**

`slice::from_raw_parts_mut`  takes a raw pointer and a length, and it creates a slice.

```rust
use std::slice;

// function doesn't need to be marked as unsafe if it contains unsafe block
// because it creates only valid pointersfrom the data the function has access to
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), // unsafe because it must trust the offset is valid
        )
    }
}
```

**Using `extern` Functions to Call External Code**

Calls to other languages are considered unsafe since they don't enforce Rust's rules and guarantees.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### Accessing or Modifying a Mutable Static Variable

Global variables in Rust are also called static variables, but they could be problematic when accessed from different threads. They are declared in screaming snake case, and must be annotated with the `static` keyword.

```rust
static HELLO_WORLD: &str = "this is a static variable";

fn main() {
    println!("static variable is: {}", HELLO_WORLD);
}
```

Difference between constants and immutable static variables:
- static variable have a fixed address in memory, whereas constants are allowed to be duplicated their data whenever they are used.
- static variables can be mutable whereas constants are immutable. Mutating a static variable is an unsafe operation.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // any code that read/write mutable static variable need to be `unsafe` block
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```