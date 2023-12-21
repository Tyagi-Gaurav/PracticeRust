use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    let opened_result = File::open("hello.txt");
    let opened_file = match opened_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //Returns Err variant io::Error, which is a struct provided by the standard library. 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok (fc) => fc,
                Err (e) => panic!("Problem opening the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        }
    };
    println!("File opened: {:?}", opened_file);
    using_closures();
    let _ = read_username_from_file();
    using_unwrap();
    using_expect();
}

fn using_closures() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("File opened again: {:?}", greeting_file);
}

fn using_expect() {
    let greeting_file = File::open("hello1.txt")
        .expect("hello.txt should be included in this project");

    //The error message used by expect in its call to panic! will be the parameter that we pass to expect.

    //In production-quality code, most Rustaceans choose expect rather than unwrap and give more context 
    //about why the operation is expected to always succeed.
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hello.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}

/*
We’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.
*/

fn read_username_from_file_2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}

fn using_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
    println!("File opened once again: {:?}", greeting_file);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}