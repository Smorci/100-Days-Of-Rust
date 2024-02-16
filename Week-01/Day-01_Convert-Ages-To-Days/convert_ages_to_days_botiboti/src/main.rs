use num_bigint::{BigUint, ToBigUint};
use std::io;

fn calc_days(age: BigUint) -> BigUint {
    let days_in_a_year: BigUint = 365_i32.to_biguint().unwrap();
    age * days_in_a_year
}

fn main() {
    println!("Enter age.");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line.");

    let age: BigUint = age.trim().parse().expect("Not an integer.");
    println!("You entered: {}.", age);
    println!("That's roughly {} days.", calc_days(age));
}
