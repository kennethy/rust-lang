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
}
