pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) /2;
    return sum * sum;
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum : u32 = 0;

    //STREAM TODO
    for i in 1..n+1 {
        sum = sum + i * i;
    }

    return sum;
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
