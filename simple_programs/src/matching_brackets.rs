pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for ch in string.chars() {
        match ch {
            '[' | '(' | '{' => stack.push(ch),
            ']' => {
                if !stack.is_empty() && stack[stack.len() - 1] == '[' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if !stack.is_empty() && stack[stack.len() - 1] == '{' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ')' => {
                if !stack.is_empty() && stack[stack.len() - 1] == '(' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => continue,
        }
    }

    return stack.is_empty();
}

//Alternate Solution
pub fn brackets_are_balanced2(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => brackets.push(c),
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    brackets.is_empty()
}