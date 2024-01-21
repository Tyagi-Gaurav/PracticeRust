use std::fs;
use std::env;
use std::error::Error;

pub fn run(config : Config) -> Result<(), Box<dyn Error>> { // Box<...> means function will return a type that implements the Error trait
    let _contents =
        fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &_contents) //Converting String to &str by using a reference.
    } else {
        search(&config.query, &_contents)
    };

    for line in results {
        println!("{line}");
    }
    
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case : bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> { //Trait bounds
        args.next(); //First value is name of the program

        let query = match args.next() {
            Some (arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some (arg) => arg,
            None => return Err("Didn't get a file_path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

//lifetime parameters specify which argument lifetime is connected to the lifetime of the return value
fn search<'a>(query: &'a str, contents : &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()    
}

fn search_case_insensitive<'a>(query: &'a str, contents : &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect()
}
 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}