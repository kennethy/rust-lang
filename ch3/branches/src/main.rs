fn main() {
    println!("Hello, world!");

    let x = 6;

    // if
    if x == 1 {
        println!("first branch");
    } else if x == 6 {
        println!("second branch");
    } else {
        println!("else branch");
    }

    // if is an expression
    let b = true;
    let b = if b { "true " } else { "false" }; // arms must have compatible types
    println!("{}", b);

    // loop
    let mut counter = 0;
    loop {
        if counter > 5 {
            break;
        }

        println!("loop");

        counter += 1;
    }

    // while
    let mut counter = 0;
    while counter < 5 {
        println!("while loop");
        counter += 1;
    }

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
