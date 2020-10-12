use std::collections::HashMap;

pub fn encode(input: &str) -> String {
    let key = key();

    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| *key.get(&c).unwrap())
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(input: &str) -> String {
    encode(input).replace(" ", "")
}

fn key() -> HashMap<char, char> {
    let mut result = HashMap::new();

    for i in 0..26 {
        result.insert((('a' as u8) + i) as char, (('z' as u8) - i) as char);
    }

    for i in 0..9 {
        let c = (('0' as u8) + i) as char;
        result.insert(c, c);
    }

    result
}
