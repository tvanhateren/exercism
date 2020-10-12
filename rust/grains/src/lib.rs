pub fn square(s: u32) -> u64 {
    match () {
        _ if s > 64 || s < 1 => panic! {"Square must be between 1 and 64"},
        _ => u64::pow(2, s - 1),
    }
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum()
}
