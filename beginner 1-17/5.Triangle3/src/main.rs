use std::io;

fn main() {
    println!("Enter size of triangle!");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read line!");
    let size: i64 = input.trim().parse().expect("Not a number");

    for i in 0..=size {
        if i%2==0 {continue;}
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
