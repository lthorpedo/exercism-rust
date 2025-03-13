pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..=n)
        .reduce(|acc, x| acc + x)
        .unwrap();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
        .reduce(|acc, x| acc + x.pow(2))
        .unwrap()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
