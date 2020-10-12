use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(score, letters)| {
            letters
                .iter()
                .map(move |letter| (letter.to_ascii_lowercase(), *score))
        })
        .collect()
}
