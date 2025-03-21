pub fn abbreviate(phrase: &str) -> String {
    phrase.split_whitespace()
        .filter(|s| s.chars().filter(|c| c.is_alphabetic()).count() > 0)
        .map(|word| acronym_from_word(word))
        .collect::<Vec<String>>()
        .join("")
}

fn acronym_from_word(word: &str) -> String {
    let has_lowercase = word.chars()
        .any(|c| c.is_ascii_lowercase());

    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .enumerate()
        .filter(|(idx, letter)| idx == &0_usize || (has_lowercase && letter.is_ascii_uppercase()))
        .map(|(_idx, letter)| String::from(letter.to_ascii_uppercase()))
        .collect::<Vec<String>>()
        .join("")
}
