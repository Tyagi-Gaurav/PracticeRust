pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    let mut i : usize = 0;
    let mut result : Vec<String> = vec![];

    while i + len <= digits.len() {
        result.push(digits[i..i+len].to_string());
        i= i + 1;
    }
    
/*
Alternate
---------
 digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|s| s.iter().collect::<String>())
        .collect()

*/

    return result;
}
