use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list.iter()
            .zip(list.iter().skip(1))
            .map(|(a, b)| format!("For want of a {a} the {b} was lost.\n"))
            .chain(once(format!("And all for the want of a {word}.")))
            .collect(),
    }
}
