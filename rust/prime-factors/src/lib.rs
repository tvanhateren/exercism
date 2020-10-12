pub fn factors(mut input: u64) -> Vec<u64> {
    let mut divisor = 2;
    let mut result = Vec::new();

    while input > 1 {
        if input % divisor == 0 {
            result.push(divisor);
            input /= divisor;
        } else {
            divisor += 1;
        }
    }

    result
}
