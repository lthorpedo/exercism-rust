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

    if number.is_empty() {
        return Ok(vec![0]);
    }

    let mut i = i32::try_from(number.len() - 1).unwrap();
    let mut base_10 = 0;
    let mut idx: usize = 0;

    while i >= 0 {
        let num = number[idx];
        if num != 0 {
            if num >= from_base {
                return Err(Error::InvalidDigit(num));
            }
            let n_u32 = u32::try_from(i).unwrap();
            base_10 += num * from_base.pow(n_u32);
        }
        i -= 1;
        idx += 1;
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

