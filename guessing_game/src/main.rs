use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generating our random number (this will be different every time the program is run)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // Initializing an empty string
        let mut guess = String::new();

        // Reading user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parsing our string a number
        // u32 => unsigned 32-bit integer
        // We ignore any input that is not of type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Comparing the generated number to the users guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
