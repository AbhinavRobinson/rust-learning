extern crate rand;

fn main() {
    // generate random number
    let r_num: i8 = rand::random();
    // switch sides
    if r_num.is_positive() {rebel_side()}
    else {dark_side()}
}

fn dark_side() {
    println!("Welcome to the dark side!");
}

fn rebel_side() {
    println!("Welcome to the rebels!");
}