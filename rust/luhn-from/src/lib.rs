pub struct Luhn {
    text: String,
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn {
            text: input.to_string(),
        }
    }
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        let value = self.text.replace(" ", "");
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
}
