pub fn rotate(input: &str, key: i8) -> String {
    assert!(key >= -26 && key <= 26);
    input
        .chars()
        .map(|c| rotate_char(c, key))
        .collect::<String>()
}

pub fn rotate_char(c: char, key: i8) -> char {
    if c.is_ascii_alphabetic() {
        let mut result = c as u8;
        let (base, end) = if c.is_ascii_uppercase() {
            ('A' as u8, 'Z' as u8)
        } else {
            ('a' as u8, 'z' as u8)
        };

        if key < 0 {
            result = result - key.abs() as u8
        } else {
            result = result + key as u8
        }

        if result < base {
            result += 26
        } else if result > end {
            result -= 26
        }

        assert!(
            (result >= 'A' as u8 && result <= 'Z' as u8)
                || (result >= 'a' as u8 && result <= 'z' as u8)
        );

        result as char
    } else {
        c
    }
}
