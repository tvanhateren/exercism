pub fn square_of_sum(num: u64) -> u64 {
    u64::pow(num * (num + 1) / 2, 2)
}

pub fn sum_of_squares(num: u64) -> u64 {
    (1..num + 1).map(|x| x * x).sum()
}

pub fn difference(num: u64) -> u64 {
    square_of_sum(num) - sum_of_squares(num)
}
