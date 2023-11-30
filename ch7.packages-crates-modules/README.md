# Chapter 7. Packages, Crates, and Modules

## 7.1. Packages and Crates

The crate root is a source file that the Rust compiler starts from and makes up the root module.

A package contains a `Cargo.toml` and is formed by one or more crates. A package must contain zero or one library crates.

A package created by the following command:

```rust
cargo new --lib [PACKAGE_NAME] // creates a binary crate with the --lib option
```

## 7.2. Defining Modules to Control Scope and Privacy

Modules let us organize code within a crate into groups for readability and easy reuse. It also controls the visibility of the items within the crate.

```rust
// src/lib.rs
mod module_name {
    mod sub_module {
        fn foo()
    }
}
```

Contents of either `src/main.rs` or `src/lib.rs`  form a module named `crate` at the root of the *module tree*.

```
crate
  - module_name
    - sub_module
      - foo
```

## 7.3. Referencing Functions within modules

A path is used to let Rust know where to find an item in a module tree. It can take two forms:

- Absolute path starts from the crate root.
- Relative path starts from current module and uses `self`, `super`, or an identifier in the current module.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// `pub` keyword is used to make a function public
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### Exposing Paths with the `pub` Keyword

By default, all items (functions, methods, structs, enums, modules, and constants) are private. Silbing modules have access to each other. Child module can refer to modules defined in the parent modules.

Making the module public doesn't make its content public. The `pub` keyword on a module only lets code in its ancestor modules refer to it.

### Relative path with `super`

Relative paths that begin in the parent module can be constructed using `super` at the start of the path (similar with the `..` syntax in a filesystem).

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // think of super as '..', to go to parent module of `back_of_house`
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public

All fields in a struct are private by default. We can make each field public or not on a case-by-case basis.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // because `seasonal_fruit` is private, we need to provide a
        // public associated function that constructs an instance of `Breakfast`.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

### Pub Enums

If an enum is public, then so are its variants.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

## 7.4. `use` Keyword

The idiomatic way is to `use` the path up to the module level when bringing in functions. Use the full path when bringing in structs, enums, and other items (if they don't have conflicting names).

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


use crate::front_of_house::hosting;
// idiomatic way 
// or use self::front_of_house::hosting;


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### 'as' keyword

Rename method with 'as' keyword.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-export with `pub use`

Use `pub use` to allow external code to use a function that was brought into the current scope

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### Using external packages

External packages are added in `Cargo.toml` and brought into scope using:

```rust
use foo::Bar;
```

### Nested Paths

```rust
// from
use std::cmp::Ordering;
use std::io;

// to
use std::{cmp::Ordering, io};
```

or

```rust
// from
use std::io;
use std::io::Write;

// to
use std::io::{self, Write}; // std::io and std::io::Write are brought into scope
```

### Glob

If we want to bring all public items defined in a path into scope, use the `*` operator (glob).

```rust
use std::collections::*;
```

## 7.5. Separating modules into different files

Using a semicolon after `mod` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

```rust
// src/front_of_house.rs
pub mod hosting; // declares a hosting module
```

Note a new folder has been created that's named `front_of_house`.

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```
