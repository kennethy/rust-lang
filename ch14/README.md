# Chapter 14. Cargo and Crates.io

## Customizing Profiles

Cargo has two main profiles. `dev` (used by `cargo build`) and `release` (used by `cargo build --release`).

Optimize using `opt-level` in `Cargo.html`, which accepts value from 0 to 3.

```toml
# Cargo.toml

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Documentation

### Generate and View Documentation

Use `cargo doc` to generate the documentation. `cargo doc --open` to geneate and open it.

### Syntax for inserting documentation
- Use `///` to add documentation.
- Use `///!` for main pages.

### Exporting a Convenient Public API with `pub use`

Use `pub use` in `lib.rs` to re-export deeply nested objects. Deeply nested objects will be documented in the main page as well.

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

### Metadata on Crate

Specify package name when publishing the crate.

```toml
[package]
name = "my_crate"
license = "MIT"
```

### Publish

Publish with `cargo publish`. The publish is permanent, and there's no way to delete published code.

### Remove Versions

Use `cargo yank --vers 1.0.1` to prevent new crates on depending the specified version. Undo with `cargo yank --vers 1.0.1 --undo`.