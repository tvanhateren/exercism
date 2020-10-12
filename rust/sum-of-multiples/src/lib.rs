use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u64, numbers: &[u64]) -> u64 {
    let mut multiples = BTreeSet::new();

    for number in numbers {
        if *number == 0 {
            multiples.insert(0);
        } else {
            let mut multiple: u64 = *number;
            while multiple < limit {
                if multiple % number == 0 {
                    multiples.insert(multiple);
                }
                multiple += *number;
            }
        }
    }

    multiples.iter().sum()
}
