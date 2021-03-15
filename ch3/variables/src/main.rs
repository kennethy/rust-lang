fn main() {
    let mut x = 5; // variables are immutable by default, less `mut` is used
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);

    // constant is different from immutable variables
    // the type of the constant variable must always be annotated
    // must be assigned to a constant expression, cannot be a result of a function call
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // shadowing
    // allows us to apply transformation, and keep the vairable immutable after
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // also allows us to change the  type
    let x = "string now";
    println!("{}", x);

    // integers
    let x: u32 = 1_000_000; // rust uses i32 by default for integers
    println!("{}", x);

    let x: i32 = 1_000_000; // i32, i64 for signed integers
    println!("{}", x);

    // floats
    let _f = 1.02; // f64 by default
    let f: f32 = 1.02;
    println!("{}", f);

    // bools
    let _b = true;
    let _b: bool = false;

    // characters
    let c = 'Z'; // chars enclosed in single quotes. 4 bytes in size. Represents a Unicode scalar value.
    println!("{}", c);

    // Compound types

    // tuple

    let t: (u32, &str, bool, char) = (1, "str", true, 'a');
    let (a, b, c, d) = t; // destructuring
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    println!("a using dot notation: {}", t.0);

    // arrays
    let _a = [1, 2, 3, 4];

    //array annotation
    let _a: [i32; 5] = [1,2,3,4,5];

    // declare an array with same values
    let a = [3; 5]; // [size; value]
    println!("{:?}", a);
}
