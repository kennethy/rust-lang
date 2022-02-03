# Chapter 16. Fearless Concurrency

The model where a language calls the operating system APIs to create threads is sometimes called `1:1` (one OS thread per one language thread).

`M:N` model is a special implementation by programming languages where `M` green threads per `N` OS threads, and they may not necessarily be equal.

Rust standard library provides an `1:1` implementation.

In the context of thread implementations, `runtime` means the code that is included by the language in every binary.

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

## Wait for all threads using `join()`

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

A channel has two halves, a transmitter and a receiver. `mpsc` stands for multiple producer, single consumer. A channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

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

`send()` returns a `Result<T, E>`.

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

`lock()` blocks the current thread until the lock is obtained. It returns a smart pointer `MutexGuard`, wrapped in a `LockResult` that we handled with the call to `unwrap`. `MutexGuard` implements the `Drop` trait and will release the lock once it becomes out of scope.

`unwrap()` is called in case the other thread holding the lock panics. This ensures the current thread will panic as well. It also unwraps the `LockResult`. 

### Multiple Ownership with Multiple Threads

`Arc` is a thread-safe alternative to `Rc`. The reason why all primitive types aren't atomic is because thread safety comes with a performance penalty that you only want to pay when you really need to.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![]; // not used in another thread, so no need to use `Arc`

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // num is a mutable reference to the value wrapped by the Mutex
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap()); // counter.lock().unwrap() works as well
}
```

### Similarities between `RefCell<T>` and `Mutex<T>`/`Arc<T>`

`Mutex<T>` provides interior mutability. This is why we are allowed to modify the `counter` even though it's immutable.

## 16.4. Extensible Concurrency with the `std::marker::Sync` and `std::marker::Send` trait

### `Send` trait

Types that implement the `Send` trait indicates that the ownership of itself can be transferred between threads. Any type composed entirely of `Send` types is automatically marked as `Send` as well.

### `Sync` trait

Types that implement `Sync` can be referenced from multiple threads. For example, `Mutex<T>` is `Sync`.

Primitives are `Sync`, and types composed entirely of `Sync` are also `Sync` automatically.

Implementing both traits manually is unsafe.