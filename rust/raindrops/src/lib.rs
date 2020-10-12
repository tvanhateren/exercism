pub fn raindrops(number: u32) -> String {
    let mut result = String::new();

    if number % 3 == 0 {
        result.push_str("Pling")
    }

    if number % 5 == 0 {
        result.push_str("Plang")
    }

    if number % 7 == 0 {
        result.push_str("Plong")
    }

    if result.len() > 0 {
        result
    } else {
        number.to_string()
    }
}
