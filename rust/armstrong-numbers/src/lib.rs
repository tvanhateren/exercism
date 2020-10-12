pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = (num as f64).log10() as u32 + 1;
    let mut value = num;
    let mut sum = 0;

    for _ in 0..num_digits {
        let digit = value % 10;
        sum += digit.pow(num_digits);
        value /= 10;
    }

    sum == num
}
