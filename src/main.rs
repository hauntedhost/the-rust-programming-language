extern crate rand;

use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;
use std::io::Write;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret);

    println!("Guess the number!");

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Not a number!");

        println!("You guessed: {}", guess);

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
