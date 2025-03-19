pub fn abbreviate(phrase: &str) -> String {
    Regex::new(r"([-\s]+)").split(phrase)
        .filter(|s| s.chars().filter(|c| c.is_alphabetic()).count() > 0)
        .map()
}
