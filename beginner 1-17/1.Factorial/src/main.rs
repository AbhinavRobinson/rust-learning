use std::io;

fn main() {
    println!("Hi! Enter your name!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Unable to parse user input.");
    println!("Hello, {}!",&input.trim());
}
