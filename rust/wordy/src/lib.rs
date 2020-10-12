use regex::Regex;

use std::str::FromStr;

pub fn answer(question: &str) -> Option<i32> {
    let math_re =
        Regex::new(r"^What is (-?\d+)((?: (?:plus|minus|multiplied by|divided by|raised to the) (?:-?\d+)(:?(?:st|nd|rd|th) power)?)+)?\?")
            .unwrap();

    if !math_re.is_match(question) {
        return None;
    }

    let captures = math_re.captures(question).unwrap();
    Some(process(
        captures
            .get(1)
            .map_or(0, |m| i32::from_str(m.as_str()).unwrap()),
        captures.get(2).map_or("", |m| m.as_str()),
    ))
}

fn process(mut a: i32, rest: &str) -> i32 {
    let operation_re = Regex::new(r"(plus|minus|multiplied by|divided by|raised to the) (-?\d+)(?:(?:st|nd|rd|th) power)?").unwrap();
    let captures = operation_re.captures_iter(rest);

    for capture in captures {
        let operation = capture.get(1).map_or("", |m| m.as_str());
        let b = capture
            .get(2)
            .map_or(0, |m| i32::from_str(m.as_str()).unwrap());

        match operation {
            "plus" => a += b,
            "minus" => a -= b,
            "multiplied by" => a *= b,
			"divided by" => a /= b,
			"raised to the" => a = (a as f64).powi(b) as i32,
            _ => panic!("Invalid operation!"),
        }
    }

    a
}
