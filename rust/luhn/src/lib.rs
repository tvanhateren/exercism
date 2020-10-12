#![feature(type_ascription)]

pub fn is_valid(value: &str) -> bool {
    let value = value.replace(" ", "");
    if value.len() <= 1 {
        return false;
    }

    let mut result = 0;

    for (index, character) in value.chars().rev().enumerate() {
        if !character.is_digit(10) {
            return false;
        }

        let mut digit = character.to_digit(10).unwrap();

        if index % 2 == 1 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }

        result += digit;
    }

    result % 10 == 0
}
