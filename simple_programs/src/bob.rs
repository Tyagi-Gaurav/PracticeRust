pub fn reply(message: &str) -> &str {
    if is_all_whitespace(message) {
        return "Fine. Be that way!";
    } else if all_caps(message) {
        if message.trim().ends_with("?") {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    } else if message.trim().ends_with("?") {
        return "Sure.";
    }
    return "Whatever.";
}

fn all_caps(message : &str) -> bool {
    message.chars().any(|x| x.is_ascii_alphabetic()) 
    &&
    message.chars()
    .filter(|x| x.is_ascii_alphabetic() && !x.is_uppercase())
    .count() == 0
}

fn is_all_whitespace(message : &str) -> bool {
    message.chars()
    .filter(|x| !x.is_ascii_whitespace())
    .count() == 0
}

//Alternate Solution
fn is_yelling(message: &str) -> bool {
    let have_letters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;
    message.to_uppercase() == message && have_letters
}

pub fn reply2(message: &str) -> &str {
    match message.trim() {
        m if m.trim().len() == 0 => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}