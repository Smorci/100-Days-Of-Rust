use num_bigint::{BigUint, ParseBigIntError};
use std::io;

const DAYS_IN_YEAR: u64 = 365;

// convert input string to BigUint
fn read_parse(inp: &str) -> Result<BigUint, ParseBigIntError> {
    let age = inp.trim().parse::<BigUint>();
    age
}

// turn age in years to days
fn calc_days(age: &BigUint) -> BigUint {
    age * DAYS_IN_YEAR
}

fn main() {
    println!("Enter age:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let age: BigUint = read_parse(&input).expect("Not a positive integer.");
    println!("You entered: {}.", age);
    println!("That's roughly {} days.", calc_days(&age));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_ordinary_human_age() {
        let result = read_parse(&String::from("50"));
        assert_eq!(result, Ok(BigUint::from(50_u32)));
    }

    #[test]
    fn age_of_earth() {
        let result = read_parse(&String::from("4500000000"));
        assert_eq!(result, Ok(BigUint::from(4500000000_u64)));
    }

    #[test]
    #[should_panic(expected = "ParseBigIntError")]
    fn not_a_positive_integer() {
        read_parse(&String::from("boo")).unwrap();
    }

    #[test]
    fn calc_days_for_earth() {
        let result = calc_days(&read_parse(&String::from("4500000000")).unwrap());
        assert_eq!(result, BigUint::from(1642500000000_u64));
    }

    #[test]
    fn calc_zero() {
        assert_eq!(
            calc_days(&read_parse(&String::from("0")).unwrap()),
            BigUint::from(0_u32)
        );
    }

    #[test]
    fn calc_one() {
        assert_eq!(
            calc_days(&read_parse(&String::from("1")).unwrap()),
            BigUint::from(365_u32)
        );
    }

    #[test]
    fn calc_hundred() {
        assert_eq!(
            calc_days(&read_parse(&String::from("100")).unwrap()),
            BigUint::from(36500_u32)
        );
    }
}
