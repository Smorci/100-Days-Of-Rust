#[allow(non_snake_case)]

pub fn findNemo(inp: &str) -> String {
    match inp.split_whitespace().position(|x| x == "Nemo") {
        Some(index) => (index + 1).to_string(),
        None => "I can't find Nemo :(".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_nemo() {
        assert_eq!(
            findNemo("We lost Nemo!"),
            String::from("I can't find Nemo :(")
        );
    }

    #[test]
    fn nemo_first() {
        assert_eq!(findNemo("Nemo is gone."), String::from("1"));
    }

    #[test]
    fn multiple_nemos() {
        assert_eq!(findNemo("Nemo Nemo Nemo"), String::from("1"));
    }

    #[test]
    fn find_nemo() {
        assert_eq!(
            findNemo("Where is nemo ?"),
            String::from("I can't find Nemo :(")
        );
    }
}
