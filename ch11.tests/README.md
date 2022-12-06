# Chapter 11. Tests

## 11.1 How to Write Tests

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
assert!(a_bool);
assert!(a_bool, "Expected, but was {}", result); // or supply custom error message
assert_eq!(a, b);
assert_ne!(a, b);
```

### Attributes

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

### Use `Result<T, E>` in Tests   

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

## 11.2. Controlling how the tests run

By default, tests are run in parallel. Use the following to configure the tests to run in sequence.

Ideally the tests should be independent so they can run in parallel without affecting each other.

```sh
cargo test -- --test-threads=1
```

### Show Function Output

By default, `println!` are captured/surpressed for passing tests. Can show all output with:
```sh
cargo test -- --show-output
```

### Run subset of tests.

```sh
cargo test [N_NAME_PATTERN]
```

Ignore tests with:
```rust
#[test]
#[ignore]
fn ignored_fn() {

}
```

Or run only the ignored tests
```sh
cargo test -- --ignored
```

## 11.3. Tests Organization

`#[cfg(test)]` tells the compiler to not include the tests module when running `cargo build`.

### Unit Tests

By convention, placed under `src/` and annotated with `#[cfg(test)]`. The convention is to create a module named `tests` in each file to contain the test funtions and to annotate the module `cfg(test)`.

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.

**Testing Private Functions**

Rust's privacy rules allow us to test private functions.

### Integration tests

By convention, they are placed in the `tests` folder at the top level of the project directory, and without the `#[cfg(test)]`. Cargo treats the `tests` folder specially and compile the crates only when running `cargo test`. Each file in the `tests` folder is compiled as its own separate crate.

Integration tests use your library in the same way any other code would.


```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

Run specific integration tests

```sh
cargo test --test [PATTERN]
```

**Submodules in Integration Tests**

To mark modules as utils modules, create a directory matching the module name and
move the util functions into the corresponding `mod.rs`, like `/tests/some_module/mod.rs`.

They would not show up when running `cargo test --test`.

**Integration Tests for Binary Crates**

Integration tests are only available for library crates, since binary crates are meant to be run on their own.