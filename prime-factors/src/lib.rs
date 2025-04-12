pub fn factors(n: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    let mut num = n;
    let mut temp = 2_u64;

    while num > 1 {
        while num % temp == 0 {
            num /= temp;
            ret.push(temp);
        }
        temp += 1;
    }

    ret
}
