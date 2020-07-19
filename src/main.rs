use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::borrow::Borrow;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("You're now playing guess the number!");
    println!("You need to try to guess a random number between 1 and 100 inclusive.");
    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Err(_) => {
                println!("Unable to read that line. Please try again.");
                continue;
            },
            _ => {}
        }

        let quit = "quit";

        if guess.eq(quit) {
            println!("Quitting");
            std::process::exit(0);
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't parse that input to a number. Please try again.");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win! Well done!"); break; },
        }
    }


}