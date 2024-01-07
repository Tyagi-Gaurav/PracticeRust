pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();

    //let mut state : [&str; minefield.len()] = [""; minefield.len()];

    for r in 0..minefield.len() {
        let row = minefield[r];
        let mut row_str : Vec<char> = Vec::new();

        for (c, char) in row.char_indices() {
            if char == ' ' {
                //Find number of mines around the space cell - (r, c)
                let count = determine_mine_count(&minefield, r, c);    
                //Push the number on row
                row_str.push(count);
            } else if char == '*' {
                row_str.push(char);
            }
        }
        result.push(String::from_iter(row_str));
    }

    return result;
}


fn determine_mine_count(minefield: &[&str], row :usize, col : usize) -> char {
    '0'
}