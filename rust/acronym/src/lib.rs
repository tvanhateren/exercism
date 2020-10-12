use regex::Regex;

pub fn abbreviate(input: &str) -> String {
    let mut result = String::new();

    let re = Regex::new(r"^[A-Z]|[ _-][A-Za-z]|[a-z][A-Z]").unwrap();

    for capt in re.captures_iter(input) {
        result.push_str(
            &if capt[0].len() == 1 {
                &capt[0][0..]
            } else {
                &capt[0][1..]
            }
            .to_uppercase(),
        )
    }

    result
}
