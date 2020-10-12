use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let input = input.split(&[' ', ',', '\n', '\t'][..]);
    let mut result: HashMap<String, u32> = HashMap::new();

    for word in input {
        let word = word.trim_matches('\'');
        let word = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>()
            .to_lowercase();

        if word.len() > 0 {
            let entry = result.entry(word).or_insert(0);
            *entry += 1;
        }
    }

    result
}
