/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars()
        .filter(|ch| !ch.is_whitespace())
        .any(|ch| !ch.is_digit(10)) {
        return false;
    }

    let mut all_digits: Vec<u32> = code
        .chars()
        .filter(|ch| ch.is_digit(10))
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();

    let len: u32 = all_digits.len() as u32; //usize to u32

    if len == 1 {
        return false;
    }

    let sum: u32 = all_digits
        .iter()
        .enumerate()
        .map(|(p, e)| {
            let index: u32 = len - p as u32 - 1;
            if index % 2 == 0 {
                *e
            } else {
                let prod = *e * 2;
                if prod > 9 {
                    prod - 9
                } else {
                    prod
                }
            }
        })
        .sum();

    return sum % 10 == 0;
}
