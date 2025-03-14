pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    
    // get number of digits by parsing string
    let power = str.len() as u32;
    
    // loop and reduce to the sum
    let total = str.chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |sum, digit| sum + digit.pow(power));
    
    num == total
}
