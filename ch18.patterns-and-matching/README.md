# 18. Patterns and Matching

## 18.1. All the Places Patterns Can Be Used

### Match Arms
```rust
match VAUE {
    PATTREN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION
}
```

`_` is used to ignore value. Match arms need to be exhaustive and account for all possible cases.

### Conditional `if let` Expressions

```rust
let favorite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    ...
} else if is_tuesday {
    ...
} else if let Ok(age) = age {
    // age is a shadwed variable only available in a new scope
    if age 30 {
        ...
    } else {
        ...
    }
}
```

`if let` does not need to be exhaustive like match arms.

### `while let`  conditional loop

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### `for` loops

```rust
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

### `let` statements

`let x = 5` actually means bind everything to the variable `x`, whatever the value is.

### Function Parameters

Function parameters can also be patterns.

```rust
fn foo(&(x, y): &(i32, i32)) {
    println!("{}, {}", x, y);
}

fn main() {
    let point = (3, 5);
    foo(&point);
}
```

## 18.2 Refutability

Patterns have two forms:
1. Irefutable: patterns that will match any value, like `let x = 5;`
2. Refutable: patterns that can fail to match for some possible values

```rust
// some_option_value could be None, but let x = y is a irrefutable pattern therefore we wrap it with `if`
// to handle the case for unmatched values
if let Some(val) = some_option_value {
    ...
}
```