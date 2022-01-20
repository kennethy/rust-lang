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