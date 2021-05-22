# 18. Patterns and Matching

## Match Arms
```
match VAUE {
    PATTREN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION
}
```

`_` is used to ignore value. Match arms need to be exhaustive and account for all possible cases.

## Conditional `if let` Expressions

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

- `If let` does not need to be exhaustive like match arms.