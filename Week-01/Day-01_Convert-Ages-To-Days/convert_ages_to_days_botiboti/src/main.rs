use num_bigint::{BigUint, ToBigUint};
use std::io;

// convert input string to BigUint
fn read_parse(inp: String) -> BigUint {
    let age: BigUint = inp.trim().parse().expect("Not a positive integer.");
    age
}

// turn age in years to days
fn calc_days(age: BigUint) -> BigUint {
    let days_in_a_year: BigUint = 365_i32
        .to_biguint()
        .expect("Something went wrong with the conversion 365: i32 -> BigUint.");
    age * days_in_a_year
}

fn main() {
    println!("Enter age.");

    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line.");

    let age: BigUint = read_parse(age);
    println!("You entered: {}.", age);
    println!("That's roughly {} days.", calc_days(age));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ordinary_human_age() {
        let result = read_parse(String::from("50"));
        let expectation: BigUint = 50_i32
            .to_biguint()
            .expect("Something went wrong with the conversion 50 : i32 -> BigUint.");
        assert_eq!(result, expectation);
    }

    #[test]
    fn age_of_earth() {
        let result = read_parse(String::from("4500000000"));
        let expectation: BigUint = 4500000000_u64
            .to_biguint()
            .expect("Something went wrong with the conversion 4.5 billion: u64 -> BigUint.");
        assert_eq!(result, expectation);
    }

    #[test]
    #[should_panic]
    fn not_a_positive_integer() {
        read_parse(String::from("boo"));
    }

    #[test]
    fn calc_days_for_earth() {
        let result = calc_days(read_parse(String::from("4500000000")));
        let expectation: BigUint = 1642500000000_u64
            .to_biguint()
            .expect("Something went wrong with the conversion 13.5 billion: u64 -> BigUint.");
        assert_eq!(result, expectation);
    }
}
