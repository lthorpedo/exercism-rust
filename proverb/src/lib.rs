pub fn build_proverb(list: &[&str]) -> String {
    let first = match list.first() {
        Some(n) => n,
        None => return String::from("")
    };

    let mut msg = list.iter()
        .zip(list.iter().skip(1))
        // .filter(|(a,b)|)
        .map(|(a, b)| format!("For want of a {a} the {b} was lost."))
        .collect::<Vec<String>>()
        .join("\n");
    if !msg.is_empty() {
        msg += "\n";
    }

    format!("{msg}And all for the want of a {first}.")
}
