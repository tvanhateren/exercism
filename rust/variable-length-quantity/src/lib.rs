#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
    SpanTooLong,
    InvalidDigit(char),
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();

    for &value in values {
        let mut value = value;
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push((value & 0x7F) as u8);
        value >>= 7;

        while value > 0 {
            bytes.push(0x80 + (value & 0x7F) as u8);
            value >>= 7;
        }

        result.extend(bytes.iter().rev())
    }

    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new();
    let mut value: u64 = 0;
    let mut done = true;

    for byte in bytes {
        value = (value << 7) + (byte & 0x7F) as u64;

        if byte & 0x80 == 0x80 {
            done = false;
        } else {
            if value > std::u32::MAX as u64 {
                return Err(Error::Overflow);
            }

            result.push(value as u32);
            value = 0;
            done = true;
        }
    }

    if done {
        Ok(result)
    } else {
        Err(Error::IncompleteNumber)
    }
}
