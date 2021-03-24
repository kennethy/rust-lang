# Chapter 7. Packages, Crates, and Modules

## Create a lib cargo with
```rust
cargo new --lib [PACKAGE_NAME]
```

## Declaration

```rust
// src/lib.rs
mod module_name {
    mod sub_module {
        fn foo()
    }
}
```

## Referencing Functions within modules

By default, all items (functions, methods, structs, enums, modules, and constants) are private.


```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

// `pub` keyword is juse to make a function public
pub fn eat_at_restaurant() {
    // Absolute path
    // crate is called the crate root
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Relative path with super
```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // think of super as '..', to go to parent
    }

    fn cook_order() {}
}
```

## Pub Structs

All fields in a struct are private by default.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
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

## Pub Enums

If an enum is public, then so are its variants.