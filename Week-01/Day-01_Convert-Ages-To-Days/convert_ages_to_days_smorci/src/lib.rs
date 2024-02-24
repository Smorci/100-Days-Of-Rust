use num_bigint::{BigUint, ParseBigIntError};
use std::io;
use std::str::FromStr;

// Number of days in a year
pub const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

// Takes a year as an argument and returns the equivalent in days
// Example:
// convert_years_to_days(1) -> 365
pub fn convert_years_to_days(years: BigUint) -> BigUint {
    NUMBER_OF_DAYS_IN_A_YEAR * years
}

// Takes a &str type and returns either the string parsed as a BigUint or an Error
pub fn convert_string_to_biguint(string: &str) -> Result<BigUint, ParseBigIntError> {
    BigUint::from_str(string.trim())
}

// Prompts the user to provide input through stdin and returns said input as String or an Error
pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

// Unit tests
#[cfg(test)]
mod tests {

    use super::*;

    // Tests a number bigger than u64::MAX
    #[test]
    fn test_big_integer() {
        let test_number = BigUint::from_str("20000000000000000000").unwrap();
        assert_eq!(
            convert_years_to_days(test_number),
            BigUint::from_str("7300000000000000000000").unwrap()
        );
    }

    // Tests a conversion from str to BigUint
    #[test]
    fn test_string_to_biguint() {
        let input = String::from("123");
        assert_eq!(
            convert_string_to_biguint(&input).unwrap(),
            BigUint::from_str("123").unwrap()
        );
    }

    // Tests a faulty input
    #[test]
    #[should_panic(expected = "ParseBigIntError")]
    fn test_conversion_parse_int_error() {
        let input = String::from("abc");
        convert_string_to_biguint(&input).unwrap();
    }
}
