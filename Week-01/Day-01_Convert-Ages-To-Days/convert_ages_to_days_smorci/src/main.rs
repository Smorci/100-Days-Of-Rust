use std::io;
use std::str::FromStr;

const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

fn convert_ages_to_days (age: u32) {

    println!("Converting ages to days...");

    let age_in_days = NUMBER_OF_DAYS_IN_A_YEAR * age;

    println!("Your age, {age}, is equal to {age_in_days} days");
}

fn get_input () -> String {

    let mut buffer = String::new();

    let stdin = io::stdin();

    let _ = stdin.read_line(&mut buffer);

    buffer
}

fn convert_string_to_integer (input: String) -> u32 {

    let input_as_unsigned_integer = u32::from_str(&input.trim()).unwrap();
    input_as_unsigned_integer

}

fn main () {

    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Please enter the number of ages to convert:");

    let input_as_string = get_input();

    let input_as_unsigned_integer: u32 = convert_string_to_integer(input_as_string);

    convert_ages_to_days(input_as_unsigned_integer);
}

