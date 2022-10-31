
use std::io;
use rand::Rng;

pub fn guessing_game() {
    println!("Guess my number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); 

    loop {
        println!("Enter your guess: ");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Ok contains num which is returned by match
            Err(_) => continue, // The _ in Err() is a catchAll value
        };

        if secret_number < guess {
            println!("Too big")
        }
        else if secret_number > guess {
            println!("Too small");
        }
        else {
            println!("You won!");
            break;
        }
    }
}