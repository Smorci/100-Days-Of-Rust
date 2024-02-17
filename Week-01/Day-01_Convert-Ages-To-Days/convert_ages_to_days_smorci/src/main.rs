use std::io;
use num_bigint::BigUint;
use std::str::FromStr;

const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

fn convert_years_to_days (years: BigUint) -> BigUint {

    NUMBER_OF_DAYS_IN_A_YEAR * years

}

fn convert_string_to_bigu32 (string: &mut String) -> BigUint {

    let result = BigUint::from_str(string.trim()).expect("Error parsing the input to unsigned integer");
    result

}

fn get_input (buffer: &mut String) -> io::Result<()> {

    let stdin = io::stdin();

    stdin.read_line(buffer)?;

    Ok(())
}

fn main () {

    let mut buffer: String = String::new();

    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Enter the number of years to convert:");
    
    let _ = get_input(&mut buffer);

    let years_as_bigu32: BigUint = convert_string_to_bigu32(&mut buffer);

    let years_in_days = convert_years_to_days(years_as_bigu32);
    let buffer_trimmed = buffer.trim();
    println!("{buffer_trimmed} years is equal to {years_in_days} days!")

}


