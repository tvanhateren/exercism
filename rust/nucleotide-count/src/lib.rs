use std::collections::HashMap;

pub fn count(nucleotide: char, dna_string: &str) -> Result<usize, char> {
    let counts;

    match nucleotide_counts(dna_string) {
        Ok(hashmap) => counts = hashmap,
        Err(message) => return Err(message),
    }

    match nucleotide {
        'A' | 'T' | 'C' | 'G' => Ok(*counts.get(&nucleotide).unwrap()),
        c => Err(c),
    }
}

pub fn nucleotide_counts(dna_string: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();
    for c in "ACGT".chars() {
        result.insert(c, 0);
    }

    for c in dna_string.chars() {
        match c {
            'A' | 'T' | 'C' | 'G' => *result.get_mut(&c).unwrap() += 1,
            c => return Err(c),
        }
    }

    Ok(result)
}
