# Chapter 14. Cargo and Crates.io

## 14.1. Customizing Profiles

Cargo has two main profiles. `dev` (used by `cargo build`) and `release` (used by `cargo build --release`).

Optimize using `opt-level` in `Cargo.html`, which accepts value from 0 to 3.

```toml
# Cargo.toml

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## 14.2. Publishing a Crate to Crates.io

### Generate and View Documentation

Use `cargo doc` to generate the documentation. `cargo doc --open` to geneate and open it.

### Syntax for Inserting Documentatiyouon
- Use `///` to add documentation comments.
- Use `//!` to add documentation on the item that contains the comment.
- Markdown is supported, so you can use `# Title` syntax to create a section

**Documentation Comments as Tests**

Running `cargo test` will run the code examples in the docuemntation as tests.

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

Use `cargo yank --vers 1.0.1` to prevent new crates on depending the specified version. Undo with `cargo yank --vers 1.0.1 --undo`. Note that yank does not delete any code.

## 14.3. Cargo Workspaces

A workspace is a set of packages that share the same `Cargo.lock` and output directory. Packages placed under the same workspace will have the compiled output placed under the same `target` folder even when we run `cargo build` in each of the package directory.

```toml
# Cargo.toml in root folder of the workspace
[workspace]

members = [
    "adder",
    "add-one",
]
```

The workspace will conists of two subdirectorys `adder`, the binary crate, and `add-one`, the lib crate. `adder/Cargo.toml` references the lib crate via:

```toml
add-one = { path = "../add-one" }
```
### Depending on an External Package in a Workspace

`Cargo.lock` will ensure all crates are using the same version of all dependencies.

### Adding a Test to a Workspace

`cargo test` runs all tests in a workspace. Alternatively, we can run package-specifc tests as well:

```sh
cargo test -p [PACKAGE_NAME]
```

Or run a specific package

```sh
cargo run -p [PACKAGE_NAME]
```

## 14.4. Installing Rust Binaries

The command will allow you to install and use binary crates locally.

```rust
cargo install [PACKAGE NAME]
```

## 14.5. Extending Cargo with custom commands

If a binary in `$PATH` is named `cargo-something`, then it can be run as a subcommand `cargo something`. Use `cargo --list` to list them all.