# Chapter 3. Common Programming Concepts

## 3.1. Variables and Mutability

Variables are immutable by default unless `mut` is used.

```rust
let x = 5;
// x = 6; compiler will error out

let mut y = 1;
y += 1; // y is mutable with the `mut` keyword
```

**Constants**

Constants are different from immutable variables. The type of the constant variable must always be annotated, and it must be assigned to a constant expression (cannot be a result of a function call).

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const MAX_POINTS: u32 = 100_000;
```

**Shadowing**

The first variable is *shadowed* by the second, which means the second variable's value is what the program sees when the variable is used.

```rust
// apply transformations and keep the variable immutable after
let x = 5;
let x = x + 1;
let x = x * 2;

// `let` creates a new variable, so we can change the type of the value and reuse the same name
let x = "string now";
```

## 3.2. Data Types

Rust is a statically typed language, so it must know the types of all variables at compile time.

### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types:
- integers
- floating-point numbers
- booleans
- characters

**Integer Types**

| length | signed | unsigned |
|--------|--------|----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

Signed numbers are stored using two's complement representation.

The `isize` and `usize` type depend on the architecture of the computer the program is running on. Integer types default to `i32` and the primary situation you'd use `isize` or `usize` is when indexing some sort of collection.

| number literals | example |
|-----------------|---------|
| decimal | 98_222 |
| hex | 0xff |
| octal | 0o77 |
| binary | 0b1111_0000 |
| byte (`u8` only) | b'A' |

Integer overflow is not checked when the program is compiled with the `--release` flag. Rust performs two's complment wrapping.

**Floating-Point Types**

```rust
let x = 2.0; // f64 by default
let y : f32 = 3.0 // f32
```

**The Boolean Type**

Booleans are one byte in size.

```rust
let t = true;
let f : bool = false;
```

**The Character Type**

Rust's `char` type is four bytes in size and represents a Unicode Scalar Value.

```rust
let c = 'z'
let z = 'ℤ'
let cat = '😻';
```

### Compound Types

Compound types group multiple values into one type. Two primitive compound types are tuples and arrays.

**Tuple**

Tuples have a fixed length: once delcared, they cannot grow/shrink in size.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup; // destructuring
println!("The value of y is : {}", y);

// or with dot notation
let first = tup.0;
```

The empty tuple `()` is a `unit type` and the value is called the `unit value`.

**The Array Type**

Arrays are useful when you want the data to be allocated on the stack rather than the heap. They are more useful when you know the number of elements will not change.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// or
let a = [1, 2, 3, 4, 5];

// shorthand
let a = [3; 5];

// for
let a = [3, 3, 3, 3, 3];

// print the array
println!("{:?}", a);
```

## 3.3. Functions

Rust doesn't care wher eyou define your functions, only that they are defined somewhere.

```rust
fn foo() {
    // ...
}
```

### Parameters

In function signatures, the type of each parameter must be declared.

### Statements and Expressions

Statements are instructions that perform some action and do not return a value. Expressions evaluate to a result value.

```rust
let y = { // a new scope block created with curly brackets is an expression
    let x = 3;
    x + 1
}
```

### Functions with Return Values

Type of return value is declared.

```rust
fn five () -> i32 {
    5 // returns 5
}
```

## 3.5. Control Flow

### `if` expressions

The condition must be a `bool`.

```rust
if number == 1 {
    // ...
} else if number == 2 {
    // ...
} else {
    // ...
}
```

### Using `if` with `let`

```rust
let number = if true { 5 } else { 6 };
```

### Repetition with Loops

**Repeating with `loop`**

```rust
loop {
    println!("again")
}
```

Loop can be labeled.

```rust
'outer: loop {
    loop {
        // ...
        break 'outer
    }
}
```

**Returning values from loops**

```rust
let mut counter = 0;

let result = loop {
    counter += 1

    if counter == 10 {
        break counter * 2;
    }
}
```

**Conditional Loops with `while`**

```rust
let mut number = 3;

while number != 0 {
    number -= 1
}
```

**Looping through a collection with `for`**

```rust
let a = [1, 2, 3];

for number in (1..4).rev() {
    println!("{}", element);
}
```
