use std::{process, env};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect(); //Collect arguments as iterator

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem processing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) { //Using let instead of unwrap as run doesn't return a value that we want to unwrap
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
