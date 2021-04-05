# 13. Functional Language Features: Iterators and Closures

## Closure

### Definition

The compiler is able to infer the typeos of the parameters and the return type. Annotate types only when we want to increase explicitness and clarity.

```rust
let closure = |num| {
    num
}
```

Annotated Version:
```rust
let closure = |num: i32| -> i32 {
    num
}
```

Cacher Example

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

## Iterators

`iter()`  returns immutable references. `into_iter()` for owned values, and `iter_mut()` for mutable references.

```rust
let v1 = vec![1, 2, 3];
let iter = v1.iter();

for i in iter {
    println!("{}", i);
}
```

### Iter Trait

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

### Next Method

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

Methods calling the `next` method are called the consuming adaptors.

```rust
let v1 = vec![1, 2, 3];
let iter = v1.iter();
let total: i32 = iter.sum(); // consumes the iter
```

### Methods that Produce Other Iterators

Iterators are lazily evaulated.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1);
```

We can manually consumed the iterator by calling `collect()`.

```rust
let v2 = v1.iter().map(|x| x + 1).collect();
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