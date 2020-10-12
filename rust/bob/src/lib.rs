pub fn reply(sentence: &str) -> &str {
    let sentence = sentence.trim();
    if sentence.is_empty() || sentence.chars().filter(|c| !c.is_whitespace()).count() == 0 {
        "Fine. Be that way!"
    } else if sentence.chars().filter(|c| c.is_alphabetic()).count() > 0
        && sentence
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
    {
        if sentence.ends_with("?") {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if sentence.ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
