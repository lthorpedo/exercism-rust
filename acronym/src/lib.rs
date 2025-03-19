use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let splitter = Regex::new(r"([-\s]+)").unwrap();
    let x: Vec<String> = splitter.split(phrase)
        .filter(|s| s.chars().filter(|c| c.is_alphabetic()).count() > 0)
        .map(|word| String::from(
            word.chars().find(|letter| letter.is_alphabetic()).unwrap().to_ascii_uppercase()
        ))
        .collect();

    x.join("")
}
