pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result : Vec<&'static str> = vec![];
    let len = diagram.len();
    let row2_start = len/2 + 1;

    let first_initial = student.chars().nth(0).unwrap() as u32 - 65;
    let std_idx : usize = (first_initial * 2).try_into().unwrap();

    result.push(get_pot_name(diagram.chars().nth(std_idx).unwrap()));
    result.push(get_pot_name(diagram.chars().nth(std_idx + 1).unwrap()));
    result.push(get_pot_name(diagram.chars().nth(row2_start + std_idx).unwrap()));
    result.push(get_pot_name(diagram.chars().nth(row2_start + std_idx + 1).unwrap()));

    return result;
}

fn get_pot_name(pot: char) -> &'static str {
    return match pot {
        'R' => "radishes",
        'G' => "grass",
        'C' => "clover",
        _ => "violets",
    }
}
