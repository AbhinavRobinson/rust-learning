extern crate rand;

use std::io;

use rand::thread_rng;
use rand::Rng;

fn main() {
    println!("Guess a number between 1-100\nPress Enter your guess:");

    // generate random
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(1..100);

    // take input
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error: Cannot readline!");
    let mut guess = user_input
        .trim()
        .parse::<u16>()
        .expect("Error: Not a Number");

    loop {
        match guess.cmp(&rand_num) {
            std::cmp::Ordering::Greater => {
                println!("Guess Lower!");
            }
            std::cmp::Ordering::Less => {
                println!("Guess Higher!");
            }
            std::cmp::Ordering::Equal => {
                println!("Correct Guess!");
                break;
            }
        }
        user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error: Cannot readline!");
        guess = user_input
            .trim()
            .parse::<u16>()
            .expect("Error: Not a Number");
    }
}
