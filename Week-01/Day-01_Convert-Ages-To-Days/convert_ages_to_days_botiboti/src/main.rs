use std::io;

fn main() {
    println!("Enter age.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You entered: {guess}");

    let days: i32 = guess.trim().parse().expect("Not an integer.");
    println!("That's roughly {} days.", days * 365);
}
