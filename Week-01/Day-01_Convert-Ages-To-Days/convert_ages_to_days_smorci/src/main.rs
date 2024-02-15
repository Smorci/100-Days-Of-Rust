use std::io;
use chrono::{NaiveDate, TimeDelta, Utc};
use std::str::Split;
use std::str::FromStr;

const NUMBER_OF_DAYS_IN_A_YEAR: u32 = 365;

struct Date {
    year: i32,
    month: u32,
    day: u32,
}

fn convert_years_to_days () {

    println!("Converting years to days...");
    println!();
    println!("Please enter the number of years to convert:");
    let years_as_string  = get_input().unwrap();

    let years_as_u32 = convert_string_to_u32(years_as_string.clone());

    let years_in_days = NUMBER_OF_DAYS_IN_A_YEAR * years_as_u32;

    println!("{years} years, is equal to {years_in_days} days", years = years_as_string.trim());
}

fn convert_string_to_u32 (input: String) -> u32 {

    let value = u32::from_str(&input.trim()).unwrap();

    value

}

fn get_input () -> io::Result<String> {

    let mut buffer = String::new();

    let stdin = io::stdin();

    let _ = stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn calculate_days_on_earth () {

    println!("Please enter your birthday (yyyy-mm-dd):");

    let birthday: String = get_input().unwrap();

    let mut input_date: Date = Date{
        year: 0,
        month: 0,
        day: 0,
    };

    let date_elements: Split<&str> = birthday.split("-");

    for (i, element) in date_elements.enumerate() {
        match i {
            0 => input_date.year = i32::from_str(&element.trim()).unwrap(),
            1 => input_date.month = u32::from_str(&element.trim()).unwrap(),
            2 => input_date.day = u32::from_str(&element.trim()).unwrap(),
            _ => panic!("Wrong input buy user! Follow instructions and respect the date format."),
        }
    }
    
    let birth_date = NaiveDate::from_ymd_opt(input_date.year, input_date.month, input_date.day).unwrap(); // NaiveDate because we dont
                                                                                                          // know the timezone

    let date_now: NaiveDate = Utc::now().date_naive();

    let duration_since_birthday: TimeDelta = NaiveDate::signed_duration_since(date_now, birth_date);

    let number_of_days_on_earth: i64 = duration_since_birthday.num_days();

    println!("You have been alive for {number_of_days_on_earth} days!")
}

fn main () {

    println!("Welcome to age converter! This program converts ages to days.");
    println!();
    println!("Select an option");
    println!("1: Convert years to days");
    println!("2: Calculate how many days I spent on earth");

    let option: &str = &get_input().unwrap();

    match option.trim() {
        "1" => convert_years_to_days(),
        "2" => calculate_days_on_earth(),
        _   => panic!("Invalid option. Please try again and enter the number of the option.")
    }

}


