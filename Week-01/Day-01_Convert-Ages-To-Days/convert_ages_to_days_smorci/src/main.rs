use num_bigint::BigUint;
use std::io;
use std::str::FromStr;

const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

fn convert_years_to_days(years: BigUint) -> BigUint {
    NUMBER_OF_DAYS_IN_A_YEAR * years
}

fn convert_string_to_biguint(string: &mut String) -> BigUint {
    let result = BigUint::from_str(string.trim()).expect("Error parsing the input to unsigned integer. Maybe the number you entered is negative, or it has alphabetical characters");
    result
}

fn get_input(buffer: &mut String) -> io::Result<()> {
    let stdin = io::stdin();

    stdin.read_line(buffer)?;

    Ok(())
}

fn main() {
    let mut buffer: String = String::new();

    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Enter the number of years to convert:");

    let _ = get_input(&mut buffer);

    let years_as_biguint: BigUint = convert_string_to_biguint(&mut buffer);

    let years_in_days = convert_years_to_days(years_as_biguint);
    let buffer_trimmed = buffer.trim();
    println!("{buffer_trimmed} years is equal to {years_in_days} days!")
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_big_integer() {
        let test_number = BigUint::from_str("20000000000000000000")
            .expect("Invalid string of digits in BigUint test");
        assert_eq!(
            convert_years_to_days(test_number),
            BigUint::from_str("7300000000000000000000")
                .expect("Invalid string of digits in BigUint test")
        );
    }

    #[test]
    fn test_string_to_biguint() {
        let mut input: String = String::from("123");
        assert_eq!(
            convert_string_to_biguint(&mut input),
            BigUint::from_str("123").expect("Invalid string of digits in conversion test")
        );
    }

    #[test]
    #[should_panic]
    fn test_conversion_parse_int_error() {
        let mut input: String = String::from("abc");
        convert_string_to_biguint(&mut input);
    }
}
