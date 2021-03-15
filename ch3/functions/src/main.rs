fn main() {
    println!("Hello, world!");

    another_function();
    that_function(1, 2);

    block();

    ret_fn();
}

fn another_function() {
    println!("Another function.");
}

fn that_function(x: i32, y: i32) { // must annotate the type of the parameters
    println!("x: {}, y: {}", x, y);
}

fn block() {
    let y = { // y is bounded to result of the block
        let x = 3;
        x + 1 // statement ends with semicolon; not needed here because it's treated as an expression
    };


    println!("y: {}", y);
}

fn ret_fn() -> i32 {
    5 // fn returns last expression in the fn implicitly, or explicitly with return
}