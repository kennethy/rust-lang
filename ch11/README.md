# Chapter 11. Tests

Test functions are annotated with `#[test]`. Run tests with `cargo test`.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```


When testing functions outside of the module, Include `use super::*` glob to bring the functions into the module.

```rust
mod tests {
    use super::*;

    #[test]
    fn test_outer() {
        outer();
    }
}

fn outer() {}
```

### Common Macros

```rust
assert!(a_boo);
assert!(a_bool, "Expected, but was {}", result); // or supply custom error message
assert_eq!(a_bool, another_bool);
assert_ne!(a_bool, another_bool);
```

### Atributes

Use `#[should_panic]` when expect fn to panic.

```rust
#[test]
#[should_panic]
fn some_fn() {
    do_something_to_trigger_panic();
}
```

To get the precise panic, use `expected`.

```rust
#[test]
#[should_panic(expected ="Panic message one")]
fn different_panic(a_val: i32) {
    if a_val == 1 {
        panic!("Panic message one");
    } else if a_val == 2 {
        panic!("Panic message two");
    }

    println!("ok");
}
```

### Use Result<T, E> in Tests   

```rust
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(());
        } else {
            Err(String::from("Failed"));
        }
    }
}
```