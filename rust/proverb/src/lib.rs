pub fn build_proverb(input: &Vec<&'static str>) -> String {
    let mut result = String::new();

    for window in input.windows(2) {
        result.push_str(&line(window[0], window[1]));
    }

    result.push_str(&match input.len() {
        0 => String::new(),
        1 | 2 => last_line(input[0]),
        _ => last_line(&format!("{}", input[0])),
    });

    result
}

fn line(a: &str, b: &str) -> String {
    format!("For want of a {} the {} was lost.\n", a, b)
}

fn last_line(a: &str) -> String {
    format!("And all for the want of a {}.", a)
}
