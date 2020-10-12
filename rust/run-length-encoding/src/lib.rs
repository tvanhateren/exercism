pub fn encode(s: &str) -> String {
    let mut result = String::new();

    if s.is_empty() {
        return result;
    }

    let mut characters = s.chars().enumerate();
    let mut last_char = characters.next().unwrap().1;
    let mut last_index = 0;

    for (index, value) in characters {
        if last_char != value {
            result.push_str(char_count_to_string(index, last_index, last_char).as_str());

            last_char = value;
            last_index = index;
        }
    }

    result.push_str(char_count_to_string(s.len(), last_index, last_char).as_str());
    result
}

fn char_count_to_string(index: usize, last_index: usize, c: char) -> String {
    let len = index - last_index;
    let len_str = if len == 1 {
        String::new()
    } else {
        len.to_string()
    };
    format! {"{}{}", len_str, c}
}

pub fn decode(s: &str) -> String {
    let mut result = String::new();

    if s.is_empty() {
        return result;
    }

    let mut characters = s.chars();
    let mut number = String::new();

    while let Some(c) = characters.next() {
        if c.is_numeric() {
            number.push(c);
        } else {
            let mut expanded_string = c.to_string();

            if !number.is_empty() {
                expanded_string = expanded_string.repeat(number.parse::<usize>().unwrap());
            }
            result.push_str(expanded_string.as_str());
            number = String::new();
        }
    }

    result
}
