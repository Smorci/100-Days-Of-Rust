use num_bigint::{BigUint, ParseBigIntError};
use std::io;
use std::str::FromStr;

const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

fn convert_years_to_days(years: BigUint) -> BigUint {
    NUMBER_OF_DAYS_IN_A_YEAR * years
}

fn convert_string_to_biguint(string: &str) -> Result<BigUint, ParseBigIntError> {

    let result = BigUint::from_str(string.trim())?;

    Ok(result)
}

fn get_input() -> io::Result<String> {

    let mut buffer = String::new();

    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;

    Ok(buffer)
}

fn main() {

    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Enter the number of years to convert:");

    let buffer: String = get_input().expect("Invalid input. Provide only a positive integer.");

    let years: BigUint = convert_string_to_biguint(&buffer).expect("Error converting string to biguint");

    let days: BigUint = convert_years_to_days(years);

    let buffer_trimmed: &str = buffer.trim();
    println!("{buffer_trimmed} years is equal to {days} days!")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_big_integer() {
        let test_number = BigUint::from_str("20000000000000000000")
            .unwrap();
        assert_eq!(
            convert_years_to_days(test_number),
            BigUint::from_str("7300000000000000000000")
                .unwrap()
        );
    }

    #[test]
    fn test_string_to_biguint() {
        let input = String::from("123");
        assert_eq!(
            convert_string_to_biguint(&input).unwrap(),
            BigUint::from_str("123").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_conversion_parse_int_error() {
        let input = String::from("abc");
        convert_string_to_biguint(&input).unwrap();
    }
}
