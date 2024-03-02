pub fn find_nemo(inp: &str) -> String {
    inp.split_whitespace()
        .position(|x| x == "Nemo")
        .map_or("I can't find Nemo :(".to_string(), |i| {
            "I found Nemo at ".to_string() + &(i + 1).to_string() + "!"
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_nemo() {
        assert_eq!(
            find_nemo("We lost Nemo!"),
            String::from("I can't find Nemo :(")
        );
    }

    #[test]
    fn just_nemo() {
        assert_eq!(find_nemo("Nemo"), String::from("I found Nemo at 1!"));
    }

    #[test]
    fn nemo_middle() {
        assert_eq!(
            find_nemo("In the Nemo is the middle."),
            String::from("I found Nemo at 3!")
        );
    }

    #[test]
    fn nemo_last() {
        assert_eq!(
            find_nemo("The last one is Nemo ."),
            String::from("I found Nemo at 5!")
        );
    }

    #[test]
    fn nemo_s() {
        assert_eq!(find_nemo("Nemo's"), String::from("I can't find Nemo :("));
    }

    #[test]
    fn multiple_nemos() {
        assert_eq!(
            find_nemo("Nemo Nemo Nemo"),
            String::from("I found Nemo at 1!")
        );
    }

    #[test]
    fn small_nemo() {
        assert_eq!(
            find_nemo("Where is nemo ?"),
            String::from("I can't find Nemo :(")
        );
    }
}
