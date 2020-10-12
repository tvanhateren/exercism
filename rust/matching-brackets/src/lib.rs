pub fn brackets_are_balanced(text: &str) -> bool {
    let mut s = Vec::new();

    for b in text.chars() {
        match b {
            '[' | '{' | '(' => s.push(b),
            ']' => {
                if s.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if s.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if s.pop() != Some('(') {
                    return false;
                }
            }
            _ => (),
        };
    }

    s.len() == 0
}
