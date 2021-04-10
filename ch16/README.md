# Chapter 16. Fearless Concurrency

## Creating Threads

```rust
use std::thread;

// ..
thread::spawn(|| {
    for i in 1..10 {
        println!("hello {} from the spawned thread", i);
    }
});
```

## Waiting using join()

`thread::spawn` returns a handle, and calling `join()` on the handle blocks the thread currently running until the thread represented by the handle terminates.

```rust
use std::thread;
use std::Duration;

fn main () {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hello {} from the spawned thread", i);
        }
    });

    handle.join().unwrap();
}
```

## Using `move` closures with threads

We need to use `move` when the closure captures variables from its surrounding scope.

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move ||{ // <---- closure takes the ownership of v
    println!("here's a vector {:?}", v);
});
```

This is to prevent `v` is always valid for the spawned thread that captured it.

```rust
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no! v is dropped because spawned thread gets to run

    handle.join().unwrap();
}
```