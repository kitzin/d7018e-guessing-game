extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("======== GUESSING GAME ========");

    // Generating a random number
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;

    loop {
    
        println!("Guess: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Could not read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Please type a number.");
                continue;
            },
        };

        // increment number of tries
        tries += 1;

        match guess.cmp(&secret) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("Yay you got it!");
                println!("It only took you {} tries.", tries);
                break;
            }
        }
    }
}
