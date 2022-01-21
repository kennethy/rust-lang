fn main() {
    // for
    let a = [1, 2, 3, 4];
    for element in a.iter() {
        println!("value is {}", element);
    }

    // range and rev
    for i in (1..4).rev() { // i exclusive
        println!("{}", i);
    }
}
