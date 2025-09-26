#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut base_10 = 0;

    for digit in number {
        if *digit >= from_base {
            return Err(Error::InvalidDigit(*digit));
        }

        base_10 *= from_base;
        base_10 += digit;
    }

    if base_10 == 0 {
        return Ok(vec![0]);
    }

    let mut n = base_10;
    let mut vec = Vec::new();

    while n > 0 {
        vec.insert(0, n % to_base);
        n /= to_base;
    }


    Ok(vec)
}

