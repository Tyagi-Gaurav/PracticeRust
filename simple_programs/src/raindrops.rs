pub fn raindrops(n: u32) -> String {
    let mut output = String::from("");
    let mut found = false;

    if n % 3 == 0 {
        output.push_str("Pling");
        found = true;
    } 
    if n % 5 == 0 {
        output.push_str("Plang");
        found = true;
    }
    if n % 7 == 0 {
        output.push_str("Plong");
        found = true;
    }

    if !found { //or output.is_empty()
        output.push_str(format!("{}", n).as_str());
    }

    return output;
}
