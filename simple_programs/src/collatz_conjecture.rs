pub fn collatz(n: u64) -> Option<u64> {
    let mut count: u64 = 0;
    let mut value: u64 = n;

    if n == 0 {
        return None;
    }

    while value != 1 {
        if value % 2 == 0 {
            value = value / 2;
        } else {
            match value.checked_mul(3) { // Alternate: n.checked_mul(3)?.checked_add(1)?
                Some(x) => match x.checked_add(1) {
                    Some(y) => value = y,
                    None => return None,
                }
                None => return None,
            }
        }
            count = count + 1;
    }

    return Some(count);
}
