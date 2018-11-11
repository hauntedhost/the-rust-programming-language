extern crate colored;
extern crate rand;
extern crate regex;

use colored::*;
use rand::Rng;
use regex::Regex;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;
use std::io::Write;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret);

    println!("Guess the number!");

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();

        let re = Regex::new(r"(?i)(quit|exit)").unwrap();
        if re.is_match(guess) {
            println!("Later.");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please input an integer!".red().bold());
                continue;
            }
        };

        // println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("Hell yeah.");
                break;
            }
        }
    }
}
