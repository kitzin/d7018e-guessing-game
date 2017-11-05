extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    // Generating a random number
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;

    // histroy vector
    let mut guesses: HashMap<u32, String> = HashMap::new();

    println!("{}", secret);
    println!("======== GUESSING GAME ========");
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


        // Append to history
        guesses.insert(tries, String::from(guess.to_string()));

        match guess.cmp(&secret) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("");
                println!("Yay you got it!");
                println!("It only took you {} tries.", tries);
                break;
            }
        }
    }

    println!("");
    print_history(&guesses);
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

fn print_history(his: &HashMap<u32, String>) {
    for (try, guess) in his.iter() {
        println!("Guess #{} was {}", try, guess);
    }
}
