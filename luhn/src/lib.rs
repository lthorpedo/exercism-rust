/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // luhn codes that are of length 1 or less are invalid
    if code.len() < 2 {
        return false;
    }

    let contains_letters = code.chars()
        .any(|c| c.is_ascii_alphabetic());
    let contains_symbols = code.chars()
        .any(|c| c.is_ascii_punctuation());
    
    if contains_letters || contains_symbols {
        return false;
    }

    const RADIX: u32 = 10;

    let mut digits = Vec::new();
    let mut digits2 = Vec::new();
    // loop over each digit (reversed / right to left)
    code.chars()
        .rev()
        .filter(|c| c.is_ascii_digit())
        .into_iter()
        .for_each(|c_dig| digits.push(c_dig.to_digit(RADIX).unwrap() as i32));

    if digits.len() < 2 {
        return false;
    }

    // double every second digit, and if it's greater than 9 then subtract 9
    for (index, n) in digits.iter().enumerate() {
        let mut temp = *n;
        if index % 2 == 1 {
            temp = n * 2;
            if temp > 9 {
                temp = temp - 9;
            }
        }
        digits2.push(temp);
    }

    let sum = digits2.into_iter()
        .reduce(|a, b| a + b)
        .unwrap();

    return sum % 10 == 0;
}






// add to settings.json to not edit rust libraries :)
// 
// "files.readonlyInclude": {
//   "**/.cargo/registry/src/**/*.rs": true,
//   "**/.cargo/git/checkouts/**/*.rs": true,
//   "**/lib/rustlib/src/rust/library/**/*.rs": true,
// },
//
