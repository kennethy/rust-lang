# Chapter 7. Packages, Crates, and Modules

Create a lib cargo with
```rust
cargo new --lib [PACKAGE_NAME]
```

Declaration

```rust
// src/lib.rs
mod module_name {
    mod sub_module {
        fn foo()
    }
}
```