pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("this is more or less squares than we have on a chessboard");
    }

    let two: u64 = 2;
    two.pow(s - 1)
}

pub fn total() -> u64 {
    (1..65)
        .reduce(|acc: u64, num: u64| acc + square(num as u32))
        .unwrap()
}
