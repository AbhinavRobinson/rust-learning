use std::io;

fn main() {
    // prompt
    println!("Select option:\n1. INR -> USD\n2. USD -> INR");

    // parse input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Unable to read input!");
    let selection: i8 = input.trim().parse().expect("Error: Not a number");

    // convert amount
    println!("{}",convert(selection));
}

fn convert(selection: i8) -> f32 {
    let mut result: f32 = 0.0;
    if selection == 1 {
        println!("Converting INR -> USD @75INR/USD\nEnter INR amount: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: Unable to read input!");
        result = input.trim().parse().expect("Error: Not a number");
        result = &result/75.0
    } else if selection == 2 {
        println!("Converting USD -> INR @75INR/USD\nEnter USD amount: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: Unable to read input!");
        result = input.trim().parse().expect("Error: Not a number");
        result = &result*75.0
    }
    result
}
