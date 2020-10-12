pub fn primes_up_to(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    };

    let mut primes = vec![true; limit + 1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=f64::sqrt(limit as f64) as usize {
        if primes[i] {
            for j in (i * i..=limit).step_by(i) {
                primes[j] = false;
            }
        }
    }

    primes
        .iter()
        .enumerate()
        .filter_map(|(index, &is_prime)| if is_prime { Some(index) } else { None })
        .collect()
}
