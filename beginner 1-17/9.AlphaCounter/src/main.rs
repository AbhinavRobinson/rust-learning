use std::io;

fn main() {
    // prompt
    println!("Enter a word to count vowels and consonents.");

    // take input
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error: Cannot readline");

    // count vowels and consonents
    let mut vows = 0;
    let mut cons = 0;
    let vowel_list = ["a", "e", "i", "o", "u"];

    // loop through the input
    for iter in user_input.trim().chars()  {
        if iter.is_ascii_digit() {
            println!("Encountered a number! Exiting..");
            break;
        } else if !iter.is_alphabetic() {
            println!("Encounterd a strange charachter! Exiting...");
            break;
        }
        let mut has_vowels = false;
        for vowel in vowel_list {
            if iter.to_string() == *vowel {
                vows+=1;
                has_vowels = true;
            }
        }
        if !has_vowels {
            cons+=1;
        }
    }

    // print out results
    println!("Vowels: {}\nConsonents: {}", vows, cons);
}
