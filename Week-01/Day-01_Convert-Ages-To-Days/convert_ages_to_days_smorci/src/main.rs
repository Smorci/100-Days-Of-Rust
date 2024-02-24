use convert_ages_to_days_smorci::{convert_years_to_days, convert_string_to_biguint, get_input};
use num_bigint::BigUint;

fn main() {
    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Enter the number of years to convert:");

    let buffer: String = get_input().expect("Unable to read input.");
    let years: BigUint = convert_string_to_biguint(&buffer)
        .expect("Could not parse year. Please enter a positive integer.");
    let days: BigUint = convert_years_to_days(years.clone());

    println!("{years} years is equal to {days} days!")
}

