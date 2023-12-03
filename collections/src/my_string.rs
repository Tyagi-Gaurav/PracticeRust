pub fn initialize() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
}

pub fn update_string() {
    //A String can grow in size and its contents can change
    let mut s = String::from("foo");

    println!("s was {s}");

    s.push_str("bar");//Appends bar to foo

    println!("s now is {s}"); 

    //Adding a single character to string
    let mut s = String::from("lo");
    s.push('l');

    println!("s now after adding a single character is {s}"); 
}

pub fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;// note s1 has been moved here and can no longer be used because add (+) takes ownership of s1.

    println!("s3 after concatenation is {s3}"); 

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("s after concatenation is {s}"); 
}

//Rust strings don’t support indexing. 

pub fn indexing_strings() {
    //Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    //You should use ranges to create string slices with caution, because doing so can crash your program
}

pub fn iterating_over_strings() {
    for c in "Зд".chars() {
        println!("Using chars(): {c}");
    }

    for b in "Зд".bytes() {
        println!("Uisng bytes(): {b}");
    }
}