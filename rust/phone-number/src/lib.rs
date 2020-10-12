struct PhoneNumber<'a> {
    country_code: &'a char,
    area_code: &'a [char],
    exchange_code: &'a [char],
    subscriber_number: &'a [char],
}

pub fn number(input: &str) -> Option<String> {
    let numbers = input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    let index = if numbers.len() == 11 {
        1
    } else if numbers.len() == 10 {
        0
    } else {
        return None;
    };

    let p = PhoneNumber {
        country_code: if numbers.len() == 11 {
            &numbers[0]
        } else {
            &'1'
        },
        area_code: &numbers[index..index + 3],
        exchange_code: &numbers[index + 3..index + 6],
        subscriber_number: &numbers[index + 6..index + 10],
    };

    if *p.country_code != '1' || p.area_code[0] < '2' || p.exchange_code[0] < '2' {
        return None;
    }

    Some(
        p.area_code
            .iter()
            .chain(p.exchange_code.iter())
            .chain(p.subscriber_number.iter())
            .collect(),
    )
}
