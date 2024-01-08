use std::ops::Div;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digit_count(num);
    let sum = get_sum(digits, num);
    return sum == num as u64;
}

fn get_sum(digits : u32, num : u32) -> u64 {
    let mut sum: u64 = 0;
    let mut n : u64 = num.into();

    while n > 0 {
        let digit: u64 = n % 10;
        sum = sum + digit.pow(digits);
        n = n.div(10);
    }

    return sum;
}

fn get_digit_count(num : u32) -> u32 {
    // let mut n = num;
    // let mut digits = 0;

    // while n > 0 {
    //     n = n.div(10);
    //     digits = digits + 1;
    // }

    // return digits;

    return num.to_string().len().try_into().unwrap();
}
