/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // luhn codes that are of length 1 or less are invalid
    if code.len() < 2 {
        return false;
    }

    const RADIX: u32 = 10;
    let nums: &[u8] = code.as_bytes();
    let mut i: i32 = (code.len() - 1) as i32;
    let mut sum: i32 = 0;
    let mut odd: bool = false;
    let mut num_digits = 0;

    while i >= 0 {
        let idx: usize = i as usize;
        if nums[idx] == b' ' {
            i -= 1;
            continue;
        }

        let num: u32 = match (nums[idx] as char).to_digit(RADIX) {
            Some(n) => n,
            None => return false,
        };

        if odd {
            let add: u32 = if num < 5 { num * 2 } else { num * 2 - 9 };
            sum += add as i32;
        } else {
            sum += num as i32;
        }

        num_digits += 1;
        i -= 1;
        odd = !odd;
    }

    if num_digits < 2 {
        return false;
    }
    
    sum % 10 == 0
}






// add to settings.json to not edit rust libraries :)
// 
// "files.readonlyInclude": {
//   "**/.cargo/registry/src/**/*.rs": true,
//   "**/.cargo/git/checkouts/**/*.rs": true,
//   "**/lib/rustlib/src/rust/library/**/*.rs": true,
// },
//
