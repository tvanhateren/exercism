const ROMAN_NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman {
    val: u32,
}

impl Roman {
    pub fn from(input: u32) -> Roman {
        Roman { val: input }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut left = self.val;
        let mut current_index = 0;

        while left > 0 {
            if ROMAN_NUMERALS[current_index].0 > left {
                current_index += 1;
            } else {
                result.push_str(ROMAN_NUMERALS[current_index].1);
                left -= ROMAN_NUMERALS[current_index].0;
            }
        }

        result
    }
}
