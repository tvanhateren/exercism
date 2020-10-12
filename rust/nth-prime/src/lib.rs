pub fn nth(num: u32) -> u32 {
    (2..).filter(|&n| is_prime(n)).nth(num as usize).unwrap()
}

fn is_prime(num: u32) -> bool {
    !(2..=((num as f64).sqrt() as u32)).any(|d| num % d == 0)
}
