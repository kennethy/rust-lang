# 13. Functional Language Features: Iterators and Closures

## 13.1. Closures

Closures are anonymous functions you can save in a variable or pass as arguments to other functions.

### Definition

The compiler is able to infer the types of the parameters and the return type. Annotate types only when we want to increase explicitness and clarity.

```rust
let closure = |num| num;
```

Annotated Version:
```rust
let closure = |num: i32| -> i32 {
    num
}
```

Cacher Example

`Fn` trait is provided by the standard library. All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = v;
                v
            }
        }
    }
}
```

There are two limitations with the Cacher implementation above:
- It assumes the `arg` that we are passing to `value` be the same, as such, the return value will always be the same.
- It only accepts closures that take one parameter of type `u32`, and return a `u32`.

### Capturing the Environment with Closures

Contrary to regular functions, closures can capture their environment and access variables from the scope in which they are defined. Additional memory is used to store the values for use in the closure body.

```rust
let x = 4;
let equal_to_x = |z| z == x;
```

Closure capture values from their environment in three ways encoded in the following traits:
- `FnOnce` consumes the variables it captures from its enclosing scope, known as closure's environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The `Once` part of the name indicates that it can't take the ownership of the same variables more than once.
- `FnMut` can change the environment because it mutably borrows values.
- `Fn` borrows values from the environment immutably.

We can force the closure to take ownership of the values by using the `move` keyword.

```rust
let x = vec![1, 2, 3];

let equal_to_x = move |z| == z == x;

// x can no longer be used since it's moved into the closure above

let y = vec![1, 2, 3];

assert!(equal_to_x(y));
```

## 13.2. Iterators

Iterators are `lazy` in Rust, which means they have no effect until you call methods that consume the iterator.

`iter()`  returns immutable references. `into_iter()` for owned values, and `iter_mut()` for mutable references.

```rust
let v1 = vec![1, 2, 3];
let iter = v1.iter();

for i in iter {
    println!("{}", i);
}
```

### The `Iterator` Trait and the `next` Method

All iterators implement the `Iterator` trait that is defined in the standard library.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

### `next` Method

`iter` needs to be mutable since it modifies the internal state of the iterator. It was not needed in a loop since the loop will take the ownership of the iter and made it mutable behind the scene.

```rust
let v1 = vec![1, 2, 3];

// needs to be mutable since calling .next() 
let mut iter = v1.iter();
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);
```

### Methods that Consume the Iterator

Methods calling the `next` method are called the consuming adaptors, because calling them uses up the iterator.

```rust
let v1 = vec![1, 2, 3];
let iter = v1.iter();
let total: i32 = iter.sum(); // consumes the iter and it cannot be used after
```

### Methods that Produce Other Iterators

Other methods defined on the `Iterator` trait, known as `iterator adaptors`, allow you to change iterators into different kinds of iterators.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1);
```

Since iterators are lazily evaulated, one of the consuming adaptor methods need to get called to get actual results. We can use the `collect()` method that consumes the iterator and collects the resulting values into a collection data type.

`map` takes in a closure and returns a new iterator.

```rust
let v2 = v1.iter().map(|x| x + 1).collect();
```

### Using Closures that Capture Their Environment

```rust
let v = vec![1, 2, 3, 4];
let v: Vec<_> = v.into_iter().filter(|num| num % 2 == 0).collect();
```

### Custom Iterators

```rust
struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { value: 0 }
    }
}

impl Iterator for Counter {
    // specify type for returned value
    type Item = u32;

    // mutable reference since the iterator will be consumed
    fn next(&mut self) -> Option<Self::Item> {
        if self.value < 5 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}
```

## 13.3.

`env::args()` returns an iterator.

## 13.4. Comparing Performance: Loops vs. Iterators

Iterators get compiled down to roughly the same code as if you'd written the lower level code yourself. Iterators are one of Rust's zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.