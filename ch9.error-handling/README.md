# Chapter 9. Error Handling

## 9.1. `panic!`

By default when `panic!` executes, the program prints a failure message, unwinds and clean up the stack (freeing memory from each functions on the stack), and then quit.

```rust
fn main() {
    panic!("crash and burn");
}
```

Rust programs can be configured to abort when panicking. This means there will be no cleanup by Rust, and it's up to the operating system to free the memory.

```toml
# Cargo.toml
[profile.release]
panic = 'abort'
```

Show backtrace by running rust programs with `RUST_BACKTRACE`. Debug symbols are enabled by default when `cargo build` or `cargo run` without the `--release` flag.

```zsh
RUST_BACKTRACE=1 cargo run
```

## 9.2. Recoverable Errors with `Result`

`Result<T, E>` enum, in which T represents the type of the value returned in the success case, and E represents the type of error returned in the failure case. Similar to `Option` enum, the `Result` enum and its variants are brought into scope by the prelude, so we do not need to specify `Result::` prefix.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Example:

```rust
let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
};
```

### Errors Matching

Using `match` can quickly bloat up the code. We can use `unwrap_or_else` with a closure to make the code more concise.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### Shortcuts

`unwrap()` returns the value in the `Ok`, and calls `panic!` for us on `Err`.

```rust
let f = File::open("hello.txt").unwrap();
```

`expect(msg: string)` similar to `unwrap()` except it allows us to specify a message that will be passed to `panic!`.

```rust
let f = File::open("hello.txt").expect("Panic Message");
```

### Propagating Errors

The function must returns a `Result` if the caller is capable of handling the error.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // return early on Err
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### Shortcut with the ? operator

The `?` operator converts the error type to ensure it matches the E type specified in the fn return `Result` type. It also early returns when there's an `Err` case.

Errors that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another.

`?` works automatically when there's an `impl From<OtherError> for ReturnedError` to define the conversion in the trait's `from` function.

The operator can only be used in functions that actually return a `Result` or `Option`.

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

Rust provides a function for this:

```rust
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}
```

## 9.3. `panic!` or not to `panic!`

Returning `Result` is a good default choice when you are define a function that might fail, because the calling code could choose to attempt to recover in a way that's appropriate for its situation.

`panic!()` or `expect()` or `unwrap()` is appropriate when:
- protoyping, testing
- we know for certain the `parse` result is valid

### Guidelines for Error Handling

Panic when program is in a bad state; when some assumption, guarantee, contract, or invariant has been broken.

