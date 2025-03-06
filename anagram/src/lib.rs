use std::collections::HashSet;

// anagrams of the given word
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort();
    possible_anagrams
        .iter()
        .filter(|a| a.len() == word.len())
        .copied()
        .filter(|a| {
            let l_a = a.to_lowercase();
            if l_a == lower_word {
                return false
            }
            let mut s: Vec<char> = l_a.chars().collect();
            s.sort();
            s == sorted_word
        })
        .collect()
}
