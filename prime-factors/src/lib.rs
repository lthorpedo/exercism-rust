pub fn factors(n: u64) -> Vec<u64> {
    let mut ret= Vec::new();
    let max = (n / 2) as u64 + 1;
    let mut num = n;
    let mut idx = 2_u64;

    while !is_prime(num) && idx < max {
        if num % idx == 0 {
            num = num / idx;
            ret.push(idx);
            idx = 1;
        }
        idx += 1;
    }

    if is_prime(num) {
        ret.push(num);
    }

    ret
}


fn is_prime(num: u64) -> bool {
    if num < 2 { return false; }
    if num == 2 { return true; }

    let max = f64::sqrt(num as f64) as u64 + 1;

    for i in 2..=max {
        if num % i == 0 { return false; }
    }

    true
}
