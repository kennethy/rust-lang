fn main() {
    // mutable string has an unknown length,
    // so it must be allocated in the heap
    let mut s = String::from("mutable string");
    s.push_str(", appended content");

    println!("{}", s);

    // in Rust, memory is automatically returned once the variable
    // that owns it go out of scope

    let y1 = String::from("str");
    let y2 = y1; // this is not allowed
    // println!("{}", y1);
    // cannot do this because y1 has been moved to y2


    let x1 = String::from("str");
    let x2 = x1.clone();
    println!("clone performs deep copy, x1 = {}, x2 = {}", x1, x2);
}
