use finding_nemo_botiboti::find_nemo;
use std::io;

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    println!("{}", find_nemo(&input));
}
