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

## Waiting using `join()`

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

## 16.2 Using Message Passing to Transfer Data Between Threads

> Do not communicate by sharing memory; instead, share memory by communicating.

### Channels

A channel has two halves, a transmitter and a receiver. `mspc` stands for multiple producer, single consumer.

```rust
use std::sync::mspc;
use std::thread;

fn main() {
    let (tx, rx) = mspc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
}
```

`recv()` blocks until a value is sent down the channel. Once it's received, `recv` returns `Result<T, E>`.

`try_recv()` is an alternative that does not block. It's useful if we want to check if there's a value available immediately.

`unwrap()` is called to ensure receiver is not closed.

### Channels and Ownerships

`send()` takes ownership of the passed variables. This is to ensure that once it's sent, the thread would not be able to use it.

### Sending Multiple Values and Seeing the Receiver Waiting

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

`rx` implements `Iterator` trait so it can be iterated. Iteration ends when the channel is closed.

### Creating Multiple Producers by Cloning the transmitter

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();

// both tx1 and tx can be used in threads to send values
```