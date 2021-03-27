# Chapter 9. Error Handling

## Unrecoverable Errors

```rust
fn main() {
    panic!("crash and burn");
}
```

Rust programs can be configured to abort when panicking

```rust
// Cargo.toml
[profile.release]
panic = 'abort'
```

Show backtrace by running rust programs with `RUST_BACKTRACE`. Debug symbols are enabled by default when `cargo build` or `cargo run` without the `--release` flag.
```
RUST_BACKTRACE=1 cargo run
```

## Recoverable Errors

`Result` enum, in which T represents the type of the value returned in the success case, and E represents the type of error returned in the failure case.

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

Errors matching with `unwrap_or_else` with closure.

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

`unwrap()`  returns the value in the `Ok`, and calls `panic!` for us on `Err`.

```rust
let f = File::open("hello.txt").unwrap();
```

`expect(msg: string)`  calls panic! 

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

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
```

Rust provides a function for this
```rust
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}
```


### Guidelines for Error Handling

Panic when program is in a bad state; when some assumption, guarantee, contract, or invariant has been broken.

