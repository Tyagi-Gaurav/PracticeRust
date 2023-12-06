use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut expected_words = Vec::new();

    for pa in possible_anagrams {
        if pa.len() != word.len() || pa == &word || eq_with_case_diff(pa, &word) {
            continue;
        }

        if is_anagram2(*pa, word) {
            expected_words.push(*pa);
        }
    }

    return expected_words.iter().cloned().collect();
}

fn eq_with_case_diff(input: &str, word : &str) -> bool {
    return input.to_lowercase() == word.to_lowercase();
}

fn is_anagram2(input: &str, word: &str) -> bool {
    //Sort and compare letters
    //Filter out the words which have same letters as uppercase

    let mut input_chars: Vec<char> = input.to_lowercase().chars().collect();
    input_chars.sort_by(|a, b| b.cmp(a));

    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_by(|a, b| b.cmp(a));

    return input_chars == word_chars;
}

fn is_anagram(input: &str, word: &str) -> bool {
    let mut word_score = HashMap::new();
    let mut input_score = HashMap::new();

    for ch in word.chars() {
        let new_ch = lower(ch);
        let count = word_score.entry(new_ch).or_insert(0);
        *count += 1;
    }

    for ch in input.chars() {
        let new_ch = lower(ch);
        let count = input_score.entry(new_ch).or_insert(0);
        *count += 1;
    }

    return word_score == input_score;
}

fn lower(ch: char) -> char {
    let ascii_code = ch as u32;
    if ascii_code >= 97 {
        return char::from_u32(ascii_code - 32).unwrap_or(ch);
    }

    return ch;
}
