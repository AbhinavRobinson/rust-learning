use std::io;

fn main() {
    // prompt
    println!("Enter number to find factorial:");

    // get and parse number
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Unable to parse user input.");
    let n:i64 = input.trim().parse().expect("Error: Not a number.");

    // find factorial! max(20)
    if n > 20 {println!("Cannot be larger than 20")}
    else {println!("{}! = {}",&n,factorial(&n))} 
}

fn factorial(n: &i64) -> i64 {
    // if less than 1
    if *n <= 0 { return 0 };
    // if 1
    if *n == 1 { return 1 };
    // else n * n-1
    *n * factorial(&(n-1))
}
