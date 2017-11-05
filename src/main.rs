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
        let guess: u32 = match read_guess() {
            Ok(num) => num,
            Err(err)  => {
                println!("{}, in parsing u32.", err);
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


fn read_guess() -> Result<u32, String> {
    let mut guess = String::new();
    let ok = match io::stdin().read_line(&mut guess) {
        Ok(_) => { true },
        Err(_) => { false },
    };

    if !ok { 
        return Err(String::from("Could not read line."));
    }

    match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(_)  => Err(String::from("Not a number")),
    }
}
