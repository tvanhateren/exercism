pub fn encode(mut number: u64) -> String {
    if number == 0 {
        return String::from("zero");
    }

    let quintillions = number / 1e18 as u64;
    number -= quintillions * 1e18 as u64;
    let quadrillions = number / 1e15 as u64;
    number -= quadrillions * 1e15 as u64;
    let trillions = number / 1e12 as u64;
    number -= trillions * 1e12 as u64;
    let billions = number / 1e9 as u64;
    number -= billions * 1e9 as u64;
    let millions = number / 1e6 as u64;
    number -= millions * 1e6 as u64;
    let thousands = number / 1_000;
    number -= thousands * 1_000;
    let hundreds = number / 100;
    number -= hundreds * 100;
    let tens = number / 10;
    number -= tens * 10;
    // number = ones

    let mut result = Vec::new();

    if quintillions > 0 {
        result.push(encode(quintillions) + " quintillion")
    }
    if quadrillions > 0 {
        result.push(encode(quadrillions) + " quadrillion")
    }
    if trillions > 0 {
        result.push(encode(trillions) + " trillion")
    }
    if billions > 0 {
        result.push(encode(billions) + " billion")
    }
    if millions > 0 {
        result.push(encode(millions) + " million")
    }
    if thousands > 0 {
        result.push(encode(thousands) + " thousand")
    }
    if hundreds > 0 {
        result.push(encode(hundreds) + " hundred")
    }

    if tens < 2 {
        let value = tens * 10 + number;
        if value > 0 {
            result.push(encode_ones(value))
        }
    } else {
        if number > 0 {
            result.push(format!("{}-{}", encode_tens(tens), encode_ones(number)))
        } else {
            result.push(encode_tens(tens))
        }
    }

    result.join(" ")
}

fn encode_ones(number: u64) -> String {
    match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
    .to_string()
}

fn encode_tens(number: u64) -> String {
    match number {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
    .to_string()
}
