use convert_ages_to_days_botiboti::calc_days;
use num_bigint::BigUint;
use std::io;

fn main() {
    println!("Enter age:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let age: BigUint = input.trim().parse().expect("Not a positive integer.");

    println!("You entered: {}.", age);
    println!("That's roughly {} days.", calc_days(&age));
}
