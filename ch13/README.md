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