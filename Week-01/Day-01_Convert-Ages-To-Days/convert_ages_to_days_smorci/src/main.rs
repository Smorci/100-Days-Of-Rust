use convert_ages_to_days_smorci::convert_years_to_days;
use num_bigint::BigUint;
use std::io;
use std::str::FromStr;

fn main() {
    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Enter the number of years to convert:");

    let mut buffer: String = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Unable to read input.");

    let years: BigUint = BigUint::from_str(buffer.trim())
        .expect("Could not parse year. Please enter a positive integer.");
    let days: BigUint = convert_years_to_days(&years);

    println!("{years} years is equal to {days} days!")
}
