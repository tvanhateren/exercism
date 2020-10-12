#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(value: &str, window: usize) -> Result<u32, Error> {
    if window > value.len() {
        return Err(Error::SpanTooLong);
    }

    if window == 0 {
        return Ok(1);
    }

    let digits = value
        .chars()
        .map(|c| to_digit(c))
        .collect::<Result<Vec<u32>, Error>>()?;

    Ok(digits
        .windows(window)
        .map(|window| window.iter().product())
        .max()
        .unwrap())
}

fn to_digit(c: char) -> Result<u32, Error> {
    c.to_digit(10).ok_or(Error::InvalidDigit(c))
}
