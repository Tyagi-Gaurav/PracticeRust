pub fn nth(n: u32) -> u32 {
    let mut i = 0;
    let mut candidate : u32 =  2;

    while i <= n {
        if is_prime(candidate) {
            if i == n {
                return candidate;
            }
            i = i + 1;
        }
        candidate = candidate + 1;
    }

    return candidate;
}

fn is_prime(n : u32) -> bool {
    //STREAM TODO
    for i in 2..(n/2+1) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}