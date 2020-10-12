#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    let invalid_digits = number
        .iter()
        .filter(|&&val| val >= from_base)
        .collect::<Vec<&u32>>();

    if invalid_digits.len() > 0 {
        return Err(Error::InvalidDigit(*invalid_digits[0]));
    }

    let number_value = number
        .iter()
        .rev()
        .enumerate()
        .map(|(index, n)| n * u32::pow(from_base, index as u32))
        .sum();

    if number_value == 0 {
        return Ok(vec![0]);
    }

    let mut value_left = number_value;
    let max_power = f64::log(number_value as f64, to_base as f64) as usize;
    let mut power = max_power;
    let mut result = vec![0; (max_power + 1) as usize];

    while value_left > 0 {
        let cur_value = u32::pow(to_base, power as u32);

        if cur_value > value_left {
            power -= 1;
        } else {
            result[max_power - power] += 1;
            value_left -= cur_value;
        }
    }

    Ok(result)
}
