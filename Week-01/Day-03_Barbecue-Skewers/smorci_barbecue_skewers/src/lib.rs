pub fn count_skewers(buffer: &str) -> [usize; 2] {
    let mut skewers = skewer_utils::parse_skewers_to_vec(&buffer);
    let mut skewers_clone = skewers.clone();
    [skewer_utils::count_vegetarians(&mut skewers), skewer_utils::count_meats(&mut skewers_clone)]
}

mod skewer_utils {
    pub fn parse_skewers_to_vec(buffer: &str) -> Vec<&str> {
        Vec::from_iter(buffer.split(','))
    }
    
    pub fn count_vegetarians(skewers: &mut Vec<&str>) -> usize {
        skewers.retain(|&skewer| !skewer.contains("x"));
        skewers.len()
    }
    
    pub fn count_meats(skewers: &mut Vec<&str>) -> usize {
        skewers.retain(|&skewer| skewer.contains("x"));
        skewers.len()
    }
    
}
