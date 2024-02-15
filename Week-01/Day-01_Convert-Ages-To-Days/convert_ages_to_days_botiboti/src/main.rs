use std::io;

fn main() {
    println!("Enter age.");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line.");

    println!("You entered: {age}");

    let days: i32 = age.trim().parse().expect("Not an integer.");
    println!("That's roughly {} days.", days * 365);
}
