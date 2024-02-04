use std::collections::HashMap;

#[derive(Debug)]
struct Letter {
    v: char,
    is_leading: bool,
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    //Parse input and letter expressions in input and output array so they can be evaluated.
    let input_binding = String::from_iter(
        input
            .chars()
            .take_while(|ch| *ch != '=')
            .filter(|ch| !ch.is_whitespace()),
    );

    let parsed_input: Vec<_> = input_binding.split("+").collect();
    let parsed_output = String::from_iter(
        input
            .chars()
            .skip_while(|ch| *ch != '=')
            .skip(2)
            .filter(|ch| !ch.is_whitespace()),
    );

    // let all_letters = get_letters(&parsed_input, &parsed_output);
    let all_letters = get_letters(&parsed_input, &parsed_output);
    //Get leading digit of all multi_digit numbers
    //Before applying a combination, check they're not 0.

    /*
        If input array is single letter and output is single letter then None.
        Get letters from input and output
        If letters more than 10, then None.
        For each letter, create a HashMap with letter and digit.
        Apply the map to words of input and output


        //For given n number of letters, calculate subsets of numbers between 0..9, taken n at a time.
        //For each combination, apply each permutation of that number.
    */
    // println!("All letters: {:?}", all_letters);


    let mut current_candidates : Option<(String, Vec<u8>)> = get_initial_combination(all_letters.len()); 

    loop {
        //For each comb in next_comb_set, calculate permutation and apply it.
        // println!("Next Combination: {:?}", current_candidates);
        match current_candidates {
            Some(ref mut x) => {
                let next_permutations = permutation(&mut x.1, 0);
                // println!("Next Permutation: {:?}", next_permutations);
                let collect : Vec<HashMap<char, u8>> = next_permutations.into_iter()
                    .map(|v| assign_to_letters(&v, &all_letters))
                    .filter(|hm| {
                        match hm {
                            Some(x) => {
                                let input_sum = calculate_input_value(&parsed_input,    x);
                                let output_sum = calculate_output_value(&parsed_output, x);
                                input_sum == output_sum
                            },
                            None => false
                        }
                    })
                    .filter(|o| o.is_some())
                    .map(|hm| hm.unwrap().to_owned())
                    .collect();
                
                if !collect.is_empty() {
                    // println!("Next Combination: {:?}, Input Sum: {}, Output_sum: {}", x.1, input_sum, output_sum);
                    return collect.get(0).cloned();
                }
            }
            None => return None,
        };
        current_candidates = get_next_combination(current_candidates.as_mut().unwrap().to_owned().0);
    }
    None
}

fn assign_to_letters(combination : &Vec<u8>, all_letters: &Vec<Letter>,) -> Option<HashMap<char, u8>> {
    let mut char_hash: HashMap<char, u8> = HashMap::new();
    for (pos, ch) in all_letters.into_iter().enumerate() {
        if all_letters.get(pos).unwrap().to_owned().is_leading && combination[pos] == 0 {
            return None
        }

        char_hash.insert((*ch).v, combination[pos]);
    }

    Some(char_hash)
}

fn calculate_input_value(parsed_input: &Vec<&str>, value: &HashMap<char, u8>) -> u64 {
    let input_sum: u64 = parsed_input
        .iter()
        .map(|f| {
            let mut sum: u64 = 0;
            for (_, ch) in f.chars().into_iter().enumerate() {
                sum = sum * 10;
                sum = sum + value_of(&value, ch);
            }
            sum
        })
        .sum();
    input_sum
}

fn calculate_output_value(parsed_output: &String, value: &HashMap<char, u8>) -> u64 {
    let mut sum: u64 = 0;
    let _output_vec: Vec<char> = Vec::new();

    for (_, ch) in parsed_output.chars().into_iter().enumerate() {
        sum = sum * 10;
        sum = sum + value_of(&value, ch);
    }

    sum
}

fn value_of(val: &HashMap<char, u8>, ch: char) -> u64 {
    // println!("Getting value of {} from {:?}", ch, val);
    *val.get_key_value(&ch).unwrap().1 as u64
}

fn get_letters(parsed_input: &Vec<&str>, parsed_output: &String) -> Vec<Letter> {
    let mut input_letters: Vec<char> = parsed_input.iter().flat_map(|f| f.chars()).collect();
    let leading_input_letters: Vec<char> = parsed_input
        .iter()
        .filter(|f| f.len() > 1)
        .map(|w| w.chars().nth(0).unwrap().to_owned())
        .collect();
    // println!("Leading input letters: {:?}", leading_input_letters);

    let mut all_letters: Vec<char> = Vec::new();
    all_letters.append(&mut input_letters);

    let mut output_letters: Vec<char> = parsed_output.chars().into_iter().collect();
    all_letters.append(&mut output_letters);

    let leading_output_letter = parsed_output.chars().nth(0).unwrap().to_owned();

    // println!("Leading output letters: {:?}", leading_output_letter);

    all_letters.sort();
    all_letters.dedup();

    return all_letters
        .into_iter()
        .map(|c| Letter {
            v: c,
            is_leading: leading_input_letters.contains(&c) || leading_output_letter == c,
        })
        .collect();

    //all_letters
}

fn get_next_combination(holder_as_string: String) -> Option<(String, Vec<u8>)> {
    let mut holder : Vec<u8> = holder_as_string.chars().into_iter().map(|ch| ch as u8 - 48).collect();
    let mut start_index = holder.len() - 1;
    let mut end = false;
    let len = holder.len();

    // println!("Calculating Next combination for : {:?}", holder.to_vec());
    while holder[start_index] == 10 - len as u8 + start_index as u8 && !end {
        if start_index == 0 {
            end = true;
        } else {
            start_index = start_index - 1;
        }
    }

    if end {
        return None;
    }

    holder[start_index] = holder[start_index] + 1;
    start_index = start_index + 1;

    while start_index < len {
        holder[start_index] = holder[start_index - 1] + 1;
        start_index = start_index + 1;
    }

    let as_string = holder.clone().into_iter()
    .map(|d| (d + 48) as char)
    .collect();
    // println!("Found Next combination for : {:?} with string: {}", holder.to_vec(), as_string);

    return Some((as_string, holder));
}

fn permutation(input: &mut Vec<u8>, start: usize) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = Vec::new();
    let index: usize = start;
    let mut j: usize = 0;

    if start + 1 == input.len() {
        output.push(vec![input.last().unwrap().to_owned()]);
        //println!("Returning Output: {:?}", output);
        return output;
    }

    while index + j < input.len() {
        input.swap(index, index + j);
        //println!("Input after swap {:?}", &input);
        let mut temp_output = permutation(&mut input.clone(), start + 1);
        //println!("Merging element {}", input[index]);
        merge(&mut temp_output, input[index]);

        output.append(&mut temp_output);
        j = j + 1;
        //println!("Value of output: {:?}", &output);
    }

    output
}

fn merge(temp_output: &mut Vec<Vec<u8>>, element: u8) -> () {
    temp_output.into_iter()
    .for_each(|v| (*v).insert(0, element));
}

fn get_initial_combination(len : usize) -> Option<(String, Vec<u8>)> {
    let mut output : Vec<u8> = Vec::new();

    for i in 0..len {
        output.push(i as u8);
    }

    let as_string = output.clone().into_iter()
    .map(|d| (d + 48) as char)
    .collect();

    Some((as_string, output))
}