pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let sentence = sentence.to_uppercase();

    alphabet.chars().all(|c| sentence.contains(c))
}
