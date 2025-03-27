pub fn nth(n: u32) -> u32 {
    let mut counter = n as i32;
    let mut idx = 1_u32;

    while counter > -1 {
        idx += 1;
        if is_prime(idx) { counter -= 1; }
    }

    idx
}

fn is_prime(num: u32) -> bool {
    if num < 2 { return false; }
    if num == 2 { return true; }

    let max = f64::sqrt(num as f64) as u32 + 1;

    for i in 2..=max {
        if num % i == 0 { return false; }
    }

    true
}
