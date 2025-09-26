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

    println!("base10: {0}", base_10);
    println!("from_base: {0}. to_base: {1}.", from_base, to_base);

    if base_10 == 0 {
        return Ok(vec![0]);
    }

    let mut power = 0;
    let mut x = to_base.pow(power);

    while x <= base_10 {
        power += 1;
        x = to_base.pow(power);
    }

    println!("power: {0}", power);

    let mut sliding = base_10;
    let mut vec = Vec::new();

    while power > 0 {
        power -= 1;
        let mut digit = 0;
        let mut subtraction = 0;
        while digit < to_base {
            digit += 1;
            let test = digit * to_base.pow(power);

            if test > sliding {
                digit -= 1;
                break;
            }

            subtraction = test;

            if test == sliding {
                break;
            }
        }

        if subtraction > 0 {
            vec.push(digit);
            sliding -= subtraction;
        }
        else {
            vec.push(0);
        }
    }

    return Ok(vec);
}

/*
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
*/


