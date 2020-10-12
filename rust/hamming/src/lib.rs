pub fn hamming_distance(a: &str, b: &str) -> Option<usize> {
    if a.len() != b.len() {
        None
    } else {
        Some(a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count())
    }
}
