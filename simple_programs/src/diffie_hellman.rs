use std::{time::{SystemTime, UNIX_EPOCH}, u8::MAX};

pub fn private_key(p: u64) -> u64 {
    let start = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("")
        .as_millis();

    let pk = 2 + start % (p - 1) as u128;
    return pk as u64;
}

//gᵃ mod p
/*

The following computations are equivalent

let r2 = g * (g * (g * (g * (g % p) % p) % p) % p) % p;
let r3 = g.pow(5) % p;

For a given (g * (g %p ) %p) -> when p > g, this is equivalent to g * g, so we're fine doing mod at every stage
*/
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modulo(a, g, p)
}

//s = Bᵃ mod p
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modulo(a, b_pub, p)
}

fn modulo(a: u64, g: u64, p: u64) -> u64 {
    if a == 1 { return 0 }
    let mut result = 1;
    let mut base = g;
    let mut exp = a;

    //This loop only runs for the number of bits in the exponent.
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % p;
        }
        exp = exp >> 1;
        base = base * base % p; //Multiplies and reduces the value of base to be used later
    }

    //BELOW is going to produce the same result as above, however its going to run many more times than the above loop.
    // for i in 1..=a {
    //     result = result * base % p;
    // }

    result
}