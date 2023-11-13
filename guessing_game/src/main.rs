use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //Unless otherwise specified, Rust defaults to an i32 - unsigned integer.

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); //Mutable variable. new is an associated function of the String type.
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) { //A match expression is made up of arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
