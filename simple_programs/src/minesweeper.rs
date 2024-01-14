pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    //let mut state : [&str; minefield.len()] = [""; minefield.len()];

    for r in 0..minefield.len() {
        let row = minefield[r];
        let mut row_str: Vec<char> = Vec::new();

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

fn determine_mine_count(minefield: &[&str], row: usize, col: usize) -> char {
    let mut sum : u8 = 0;

    if let Some(prev_row) = row.checked_sub(1) {
        if let Some(prev_col) = col.checked_sub(1) {
            sum = sum + countAt(minefield, prev_row, prev_col);
        }
        sum = sum + countAt(minefield, prev_row, col);
        if let Some(next_col) = col.checked_add(1) {
            sum = sum + countAt(minefield, prev_row, next_col);
        }
    }

    if let Some(prev_col) = col.checked_sub(1) {
        sum = sum + countAt(minefield, row, prev_col);
    }

    if let Some(next_col) = col.checked_add(1) {
        sum = sum + countAt(minefield, row, next_col);
    }

    if let Some(next_row) = row.checked_add(1) {
        if let Some(prev_col) = col.checked_sub(1) {
            sum = sum + countAt(minefield, next_row, prev_col);
        }
        sum = sum + countAt(minefield, next_row, col);
        if let Some(next_col) = col.checked_add(1) {
            sum = sum + countAt(minefield, next_row, next_col);
        }
    }

    if sum == 0 {
        return ' ';
    } else {
        return (48 + sum) as char;
    }
}

fn countAt(minefield: &[&str], row: usize, col: usize) -> u8 {
    match valueAt(minefield, row, col) {
        Some('*') => 1,
        _ => 0,
    }
}

fn valueAt(minefield: &[&str], row: usize, col: usize) -> Option<char> {
    minefield.get(row)?.chars().nth(col)
}


//Alternate

/*
static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len() as i32;
    (0..height).map(|y| {
        let width = field[y as usize].len() as i32;
        (0..width).map(|x| {
            if field[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                match NEIGBOURHOOD_OFFSETS.iter()
                    .map(|&(ox, oy)| (x + ox, y + oy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                        0 => ' ',
                        n => (n as u8 + '0' as u8) as char
                    }
            }
        }).collect()
    }).collect()
}
*/