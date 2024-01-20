use rand::Rng;
use std::cmp::Ordering;
use std::io; // Import standard in-out library.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess."); // Prompt for input.

        let mut guess = String::new(); // Create mutable variable named guess, give it type String. ::new() associated function gives empty string.

        io::stdin() // in-out::standard-in recieves input, if not imported at top: std::io::stdin() would work.
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handles a failure, not strictly necessary but you will be warned during compile that error is possible.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // Match to the ordering output for response.
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
