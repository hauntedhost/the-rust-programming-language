extern crate colored;

use colored::*;

fn main() {
    let x = {
        let y = five();
        y + (y * 2) // no semicolon dawg
    };

    let mut y = 1;
    println!("let's explore show_xy in a weird while loop:");
    while y <= 9 {
        show_xy(x, y);
        y = y + 1;
    }

    println!("\nhow about a for loop over an array:");
    let a = [10, 20, 25, 30, 50, 75];
    for n in a.iter() {
        show_xy(*n, n * n);
    }

    println!("\nlet's redo our first while as a for over a range:");
    for y in 1..9 {
        show_xy(x, y);
    }
}

fn five() -> i32 {
    5
}

fn is_lucky(n: i32) -> bool {
    n == 7
}

fn show_xy(x: i32, y: i32) {
    println!("the value of 'x' is {}", x);
    show_y(y);
}

fn show_y(y: i32) {
    // if is an expression 😎
    let message = "the value of 'y' is";
    let (message, y) = if is_lucky(y) {
        (message.green(), y.to_string().green())
    } else {
        (message.normal(), y.to_string().normal())
    };
    println!("{} {}", message, y);
}
