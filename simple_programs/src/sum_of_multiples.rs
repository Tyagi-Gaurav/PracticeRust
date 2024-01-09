pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];
    //STREAM TODO
    //For each factor, find the multiples of the factor
    for f in factors {
        let mut i = *f;
        while i != 0 && i < limit {
            multiples.push(i);
            i = i + f;
        }
    }

    //println!("All factors {:?}", multiples);
    multiples.sort();
    //Create a unique array containing all factors
    let mut last_multiple : u32 = 0;
    let mut sum = 0;
    for (pos, e) in multiples.iter().enumerate() {
        if multiples[pos] != last_multiple {
            sum = sum + multiples[pos];
            last_multiple = multiples[pos];
        }
    }

    //Sum the factors
    return sum;
}
