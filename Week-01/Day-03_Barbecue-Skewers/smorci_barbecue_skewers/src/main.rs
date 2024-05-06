use std::io;
use smorci_barbecue_skewers::{count_skewers};

fn main() {
    println!("Please enter a barbecue skewer delimited by ','");

    let mut buffer: String = String::new();

    // Input from the terminal doesn't really work, because it take an Enter (newline) as the end
    // of input. Let's start working with files, more beneficial IMO
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input from stdin.");

    let skewer_count = count_skewers(&buffer);

    println!("The number of vegetarian and non-vegetarian skewers are [{},{}], respectively", skewer_count[0], skewer_count[1]);

}
