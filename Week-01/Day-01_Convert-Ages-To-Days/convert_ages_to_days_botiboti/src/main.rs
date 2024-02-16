use std::io;

fn calc(str: String) -> i32 {
    let days: i32 = str.trim().parse().expect("Not an integer.");
    days * 365
}

fn main() {
    println!("Enter age.");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line.");

    println!("You entered: {age}");

    println!("That's roughly {} days.", calc(age));
}
