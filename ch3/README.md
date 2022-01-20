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