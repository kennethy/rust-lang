use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // randomly generates a number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess!");

        // rust variables (including references) immutable by default
        // use `mut` special keyword to indicate that it's mutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // allows stdin to mutate guess
            .expect("Failed to read line");

        let guess: u32 = match guess // shadowing previously defined variable
            .trim() // removed end of line token
            .parse() { // method on string type that parses into a number
                Ok(num) => num,
                Err(_) => continue,
            };

        // read_line returns io::Result, an enum of `Ok` or `Err`
        // `Ok` returns the value
        // `Err`

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
