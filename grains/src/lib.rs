pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(s-1),
        _ => panic!("Gotta check your chessboard. Must be between 1 & 64")
    }
}

pub fn total() -> u64 {
    (1..=64)
        .fold(0,|acc: u64, num: u32| acc + square(num))
}
