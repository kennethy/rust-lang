# Chapter 15. Smart Pointers

Smart pointers not only act like a pointer, but have additional metadata and capabilities. References are pointers that only borrow data, whereas smart pointers own the data they point to. They are usually implemented using structs that implement `Deref` and `Drop` traits.

`Deref` trait allows an instance of the smart pointer struct to behave like a reference, and `Drop` trait allows you to customize the code to run when the smart pointer goes out of scope.

## 15.1. Using Box<T> to Point to Data on the Heap

`Box<T>` is a pointer stored on the stack, that references data stored in the heap.

Used when:
1. have a type whose size is unknown at compile time and you want to use a value of that type in a context that requires an exact size
2. large amount of data you want to transfer ownership but ensure the data won't be copied when you do so
3. when you want to own a value and you care only only that it's a type that implements a particular trait rather than being of a specific type.


### Using a `Box<T>` to Store Data on the Heap

```rust
let b = Box::new(5);
println!("b = {}", b);
```

### Enabling Recursive Types with Boxes

For a recursive type which size can't be known at compile can use `Box<T>`.

```rust
enum List {
    Cons(i32, List), // <---- this would not work
    Nil
}
```

However, with `Box<T>`,

```rust
enum List {
    Cons(i32, Box<List>),
    Nil
}
```

`Box<T>` provide only the indirection and heap allocation; they don't have any other special capabilities. It implements `Deref` so values can be treated like references.

## 15.2 Treating Smart Pointers like Regular References with the `Deref` Trait

Implementing the `Deref` trait allows us to customize the behaviour of the deference opreator (`*`).

### Using `Box<T>` Like a Reference

```rust
let x = 5;
let y = Box::new(x);

assert_eq!(5, x);
assert_eq!(5, *y);
```
### Defining Own Smart Pointer

To support deferencing, we need to implement the `Deref` trait with the `deref` fn for the struct.

```rust
struct MyBox<T>(T);

impl<T> Mybox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { // ->&T works as well
        &(self.0) // &self.0 works as well
    }
}
```

Rust runs `*(smart_pointer.deref())` under the scene. `deref()` returns a reference because of the ownership system. We don't want to take ownership of the value inside the box.

### Implicit Deref Coercion

Rust performs deref coercion on arguments (that implement the `Deref` Trait) to functions and methods.

Deref coercion can turn `&String` into `&str` because `String` implements the `Deref` trait such that it returns `&str`.

```rust
fn hello(name: &str) {
    ...
}

// with deref coercion
// &String -> &str
let m = MyBox::new(String::from("hello"));

// 1. pass a reference because we don't want to transfer ownership
// 2. MyBox implements deref, (*m) => *(m.deref()) => *(&String) => *(&str)
hello(&m); 


// or without deref coercion
hello(&(*m)[..]);
```

### How Deref Coercion Interacts with Mutability

Rust does deref coercion when it finds types and trait implementations in three cases:
1. From `&T` to `&U` when `T: Deref<Target = U>`
2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
3. From `&mut T` to `T: Deref<Target=U>`

## 15.3 Running Code on Cleanup with the `Drop` Trait

Implementing the `Drop` trait allows us to specify the code to run when a value goes out of scope. It requires an implementation of `fn drop(&mut self)`. The trait is included in the prelude. Variables are dropped in the reverse order of their creation.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

### Dropping a Value early with `std::mem::drop`

`drop` is always called when variables go out of scope. To prevent `double free` error, Rust does not allow us to call `drop` explicitly.

Use `std::mem::drop` instead.

```rust
// std::mem::drop is in the prelude
drop(v);
```

## 15.4 `Rc<T>`, the Reference Counted Smart Pointer

`Rc<T>`, abbreviated for reference counting, is a smart pointer type that keep tracks of the number of references to a value. It's applicable when a value has multiple owners. It's only for use in single-threaded scenarios.

We need to `use std::rc::Rc` because it's not in the prelude.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

### Cloning an `Rc<T>`

Every call to `Rc::clone()` increments the reference count. `a.clone()` is allowed the convention is to use `Rc::clone`.

Use `Rc::strong_count()` to get the reference count.

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
}
```

## 15.5 `RefCell<T>` and the Interior Mutability

With `RefCell<T>`, the borrowing rules' invariants are enforced at runtime.

Mutating the value inside an immutable value is the interior mutability pattern.

### Differences

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

### Usage

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // as opposed to Vec<String>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Messenger trait uses &self, so cannot use &mut self
            // sent_messages can still be modified with borrow_mut();
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

### Keeping Track of Borrows at Runtime with RefCell<T>

With `RefCell<>T`, `borrow()` returns a smart pointer `Ref<T>`, whereas `borrow_mut()` returns another smart pointer type `RefMut<T>`. Both implements `Deref` so they can be treated like references. `Ref<T>` keeps track of the count of borrows, and it will go out of scope when the count reaches zero. It allows many immutable borrows or one mutable borow at any point in time.

### Having multiple owners of mutable data by combining `Rc<T>` and `RefCell<T>`.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10; // *(value.borrow_mut()) += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

## 15.6. Reference Cycles

### Creating a Cycle

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // allow to change what the next element points to
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

### Preventing Reference Cycles with Weak<T>

Create a weak reference by calling `Rc::downgrade` with a reference to `Rc<T>`. It returns a smart pointer typed to `Weak<T>`. The count is tracked via `weak_count` and the difference between `strong_count` is that `weak_count` doesn't need to be 0 for the `Rc<T>` instance to be cleaned up.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // refers to parent, but does not own parent
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // need to upgrade when using the pointer

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // downgrade when assign

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```