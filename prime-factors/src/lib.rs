pub fn factors(n: u64) -> Vec<u64> {
    todo!("This should calculate the prime factors of {n}")
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

fn complete(my_vec: Vec<u64>) {
    
}