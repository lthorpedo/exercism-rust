/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // luhn codes that are of length 1 or less are invalid
    if code.len() < 2 {
        return false;
    }

    const RADIX: u32 = 10;

    let dig = ' ';
    let my_bool = dig.is_ascii_digit();
    todo!("dig {dig} {my_bool}");

    let mut digits = Vec::new();
    let mut digits2 = Vec::new();
    // loop over each digit (reversed / right to left)
    code.chars()
        .rev()
        .filter(|c| c.is_ascii_digit())
        .map(|c_dig| digits.push(c_dig.to_digit(RADIX).unwrap() as i32));

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

    todo!("what's teh sum {sum}");

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
