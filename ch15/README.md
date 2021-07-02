# Chapter 15. Smart Pointers

Smart pointers not only act like a pointer, but have additional metadata and capabilities. They are usually implemented using structs that implement `Deref` and `Drop` traits.

`Deref` trait allows an instance of the smart pointer struct to behave like a reference, and `Drop` trait allows you to customize the code to run when the smart pointer goes out of scope.

## 15.1 Using Box<T> to Point to Data on the Heap

Box<T> is a pointer stored on the stack, that references data stored in the heap.

```rust
let b = Box::new(5);
println!("b = {}", b);
```

## 15.2 Treating smart pointers like regular references with the `Deref` Trait

Box<T> is a smart pointer that implements the `Deref` trait and allow Box<T> values to be treated like references.

### Defining Own Smart Pointer

```rust
struct MyBox<T>(T);

impl<T> Mybox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T{
        &(self.0)
    }
}
```

Rut runs `*(smart_pointer.deref())` under the scene. `deref()` returns a reference because of the ownership system. We don't want to take ownership of the value inside the box.

### Implicit Deref Coercion

Rust performs deref coercion on arguments (that implements the `Deref` Trait) to functions and methods.

### How Deref Coercion Interacts with Mutability

Rust does deref coercion when it finds types and  trait implementations in three cases:
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

Rust does not allow calling `val.drop()` directly. Use `std::mem::drop` instead.

```rust
// std::mem::drop is in the prelude
drop(v);
```

## 15.4 `Rc<T>`, the Reference Counted Smart Pointer

`Rc<T>`, abbreviated for reference counting, is a  smart pointer type that keep tracks of the number of references to a value. It's applicable when a value has multiple owners. It's only for use in single-threaded scenarios.


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

We need to `use std::rc::Rc` because it's not in the prelude. Every call to `Rc::clone()` increments the reference count.

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