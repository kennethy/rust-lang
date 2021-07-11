# Chapter 16. Fearless Concurrency

The model where a language calls the operating system APIs to create threads is sometimes called `1:1` (one OS thread per one language thread).

`M:N` model is a special implementation by programming languages where `M` green threads per `N` OS threads, and they may not necessarily be equal.

Rust standard library provides an `1:1` implementation.

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

This is to ensure `v` is always valid for the spawned thread that captured it.

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

## 16.2. Using Message Passing to Transfer Data Between Threads

> Do not communicate by sharing memory; instead, share memory by communicating.

### Channels

A channel has two halves, a transmitter and a receiver. `mspc` stands for multiple producer, single consumer.

```rust
use std::sync::mpsc;
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

`send()` takes ownership of the passed variables. This is to ensure that once it's sent, the thread would not be able to use it. The receiver takes the ownership when the variables are moved.

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
thread::spawn(move || {
    tx1.send(...).unwrap();
});

thread::spawn(move || {
    // ...
    tx.send(...).unwrap();
});

for received in rx {
    // ...
}

// both tx1 and tx can be used in threads to send values
```

## 16.3. Shared State Concurrency

### Using Mutexes

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

`lock()` blocks the current thread until the lock is obtained.

`unwrap()` is called in case the other thread holding the lock panics. This ensures the current thread will panic as well.

Both calls are called in an inner scope because `lock()` returns a smart pointer `MutexGuard` then will release the lock once it becomes out of scope.

### Multiple Ownership with Multiple Threads

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc is a thread-safe alternative to Rc. `a` stands for atomic.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock.unwrap());
}
```