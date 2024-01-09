pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors : Vec<u64> = vec![];
    let mut val = n;
    let mut i = 2;

    while i <= val {
        if val % i == 0 && is_prime(i) {
            prime_factors.push(i);
            val = val / i;
            //println!("Found prime {}, new value: {}", i, val);
            i = 2;
        } else {
            i = i + 1;
        }
    }

    return prime_factors;
}

fn is_prime(n : u64) -> bool {
    //STREAM TODO
    for i in 2..(n/2+1) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}