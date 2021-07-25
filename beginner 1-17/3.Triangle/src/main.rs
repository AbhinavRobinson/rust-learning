use std::io;

fn main() {
    println!("Enter size of triangle: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line!");
    let size : i32 = input.trim().parse().expect("Not a number!");

    for i in (0..size+1).rev() {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
