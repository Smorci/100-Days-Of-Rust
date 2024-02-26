pub fn findNemo(inp: &str) {
    let nemo: &str = "Nemo";
    match inp.split_whitespace().position(|x| x == nemo) {
        Some(index) => println!("{index}"),
        None => println!("I can't find Nemo :("),
    }
}

#[cfg(test)]
mod tests {}
