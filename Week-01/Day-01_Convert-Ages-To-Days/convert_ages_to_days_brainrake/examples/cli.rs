use num::BigUint;
use std::io;
use convert_ages_to_days_brainrake::years_to_days;

fn main() {
    println!("Please enter your age in years:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    let years: BigUint = input
        .trim()
        .parse()
        .expect("Could not parse age. Please enter a positive integer.");

    let days = years_to_days(years);

    println!("You are around {days} days old.")
}
