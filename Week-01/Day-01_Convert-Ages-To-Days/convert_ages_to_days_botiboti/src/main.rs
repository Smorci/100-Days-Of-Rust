use std::io;

fn calc(age: i32) -> i32 {
    age * 365
}

fn main() {
    println!("Enter age.");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line.");

    let age: i32 = age.trim().parse().expect("Not an integer.");
    println!("You entered: {}.", age);
    println!("That's roughly {} days.", calc(age));
}
