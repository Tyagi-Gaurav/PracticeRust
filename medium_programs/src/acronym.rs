pub fn abbreviate(phrase: &str) -> String {
    let phrase_without_special_chars: Vec<char> =
        phrase.chars().into_iter().map(|ch| convert(ch)).collect();
    let temp_result = String::from_iter(phrase_without_special_chars);
    println!("Temp: {temp_result}");

    let final_result: Vec<char> = temp_result
        .as_str()
        .split(' ')
        .map(|word| word.trim())
        .filter(|word| !word.is_empty())
        .flat_map(|word| any_upper_case(word))
        .collect();

    println!("Final: {:?}", final_result);

    String::from_iter(final_result).to_uppercase()
}

fn any_upper_case(word : &str) -> Vec<char> {
    let mut start = vec![word.chars().nth(0).unwrap()];

    let mut remaining : Vec<char> = word.chars().collect::<Vec<char>>()
    .windows(2)
    .filter(|s| s[0].is_ascii_lowercase() && s[1].is_ascii_uppercase())
    .map(|s| s[1])
    .collect();
    

    start.append(&mut remaining);
    return start;
}

fn convert(ch: char) -> char {
    if ch == '_' || ch == '-' {
        ' '
    } else {
        ch
    }
}

/*
Alternative
-----------

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}

*/
