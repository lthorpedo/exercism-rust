/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // luhn codes that are of length 1 or less are invalid
    if code.chars().filter(|c| c.is_ascii_digit()).count() < 2 {
        return false;
    }

    const RADIX: u32 = 10;
    let nums: &[u8] = code.as_bytes();
    let mut i: i32 = (code.len() - 1) as i32;
    let mut sum: i32 = 0;
    let mut odd: bool = false;

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

        i -= 1;
        odd = !odd;
    }

    sum % 10 == 0
}

