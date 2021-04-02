# 13. Functional Language Features: Iterators and Closures

## Closure

### Definition

The compiler is able to infer the typeos of the parameters and the return type. Annotate types only when we want to increase explicitness and clarity.

```rust
let closure = |num| {
    num
}
```