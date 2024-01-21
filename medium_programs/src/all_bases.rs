#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
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
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    return match to_decimal(from_base, number) {
        Ok(num_in_decimal) => Ok(from_decimal(to_base, num_in_decimal)),
        Err(e) => Err(e),
    }
}

fn to_decimal(from_base : u32, number : &[u32]) -> Result<u64, Error> {
    let mut result : u64 = 0;
    let mut mult : u64 = 1;

    if !number.is_empty() {
        let mut index = number.len() - 1;

        while index > 0 {
            if number[index] >= from_base {
                return Err(Error::InvalidDigit(number[index]));
            }

            result = result + mult * number[index] as u64;
            //println!("Result: {result}");
            index = index - 1;
            mult = mult * from_base as u64;
        }

        result = result + mult * number[0] as u64
    }

    return Ok(result);
}

fn from_decimal(to_base : u32, number : u64) -> Vec<u32> {
    let mut result : Vec<u32> = Vec::new();
    let mut num = number;

    while num > 0 {
        result.insert(0, num as u32 % to_base);
        num = num / to_base as u64;
    }

    if result.is_empty() {
        [0].to_vec()
    } else {
        result
    }
}