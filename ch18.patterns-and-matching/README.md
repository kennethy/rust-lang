# 18. Patterns and Matching

## 18.1. All the Places Patterns Can Be Used

### Match Arms
```rust
match VALUE {
    PATTREN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION
}
```

`_` is used to ignore value. Match arms need to be exhaustive and account for all possible cases.

### Conditional `if let` Expressions

```rust
let favorite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    ...
} else if is_tuesday {
    ...
} else if let Ok(age) = age {
    // age is a shadwed variable only available in a new scope
    if age 30 {
        ...
    } else {
        ...
    }
}
```

`if let` does not need to be exhaustive like match arms.

### `while let`  conditional loop

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### `for` loops

```rust
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

### `let` statements

`let x = 5` actually means bind everything to the variable `x`, whatever the value is.

### Function Parameters

Function parameters can also be patterns.

```rust
fn foo(&(x, y): &(i32, i32)) {
    println!("{}, {}", x, y);
}

fn main() {
    let point = (3, 5);
    foo(&point);
}
```

## 18.2 Refutability

Patterns have two forms:
1. Irefutable: patterns that will match any value, like `let x = 5;`
2. Refutable: patterns that can fail to match for some possible values

```rust
// some_option_value could be None, but let x = y is a irrefutable pattern therefore we wrap it with `if`
// to handle the case for unmatched values
if let Some(val) = some_option_value {
    ...
}
```

## 18.3. Pattern Syntax

### Matching Literals

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
}
```

### Matched Named Variables

`match` starts a new scope, and variables introduced in the match arm would shadow variables with the same name in the outer scope.

### Multiple Patterns

Multiple patterns can be matched using the `|` syntax.

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### Matching Ranges of Values with `..=`

```rust
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

or `char` values:

```rust
match x {
    'a'..='j' => println!("a to j"),
    'k'..='z' => println!("k to z"),
    _ => println!("something else"),
}
```

### Ignoring Values in a Pattern

**Ignoring an Entire Value with `_`**

```rust
fn foo(_: i32, y: i32) {
    println!("this code only uses y: {}", y);
}
```

**Ignoring Parts of a Value with a Nested `_`**

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth);
    }
}
```

**Ignoring an Unused Variable by Starings its Name with `_`**

```rust
fn let main() {
    let _x = 5;
    let y = 10;
}
```

The syntax `_x` still binds the value to the variable.

```rust
let s = Some(String::from("hello"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s); // won't work because the string is moved into `_s`
```

`_` doesn't bind at all. 

```rust
let s = Some(String::from("hello"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s); // ok
```

**Ignoring Remaining Parts of a Value with `..`**

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

`..` can also be used to expand as many values as it needs to be.

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    }
}
```

### Extra Conditionals with Match Guards

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

Or with `|`

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

### `@` Bindings

The `@` operator allows us to create a variable that holds a value at the same time we are testing that value to see whether it matches a pattern.

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("found an id in range: {}", id_variable),
    Message::Hello { id: 10..= 12 } => {
        println!("found an id in another range")
    },
    Message::Hello { id } => println!("some other matches"),
}
```