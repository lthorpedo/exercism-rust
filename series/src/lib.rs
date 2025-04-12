pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() || digits.is_empty() { return Vec::new(); }

    let mut series = Vec::new();
    let mut idx = 0_usize;

    while idx + len <= digits.len() {
        let serie = String::from_iter(digits.chars().skip(idx).take(len));
        series.push(serie);
        idx += 1;
    }
    
    series
}
