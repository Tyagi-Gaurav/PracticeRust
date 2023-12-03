use std::vec; //A Macro

pub fn vector() {
    let v : Vec<i32> = Vec::new();

    //Another vector using a macro
    let v_m = vec![1, 2, 3];

    println!("First element Vector with macro: {}", v_m[0]);
}

pub fn update_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Second element Vector after update: {}", v[1]);
}

pub fn read_values() {
    let v = vec![1, 2, 3];

    let third: &i32 = &v[2]; //reference to vector element
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

pub fn iterate_over_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

pub fn hold_different_types_in_vec() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}