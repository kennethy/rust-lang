fn main() {
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
