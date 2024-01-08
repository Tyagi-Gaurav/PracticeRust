use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub mod anagram;
pub mod clock;
pub mod space_age;
pub mod sublist;
pub mod minesweeper;
pub mod beer;
pub mod armstrong;


pub fn gigasecond_after_start_time(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    return start + Duration::seconds(1_000_000_000);
}

pub fn reverse(input: &str) -> String {
    let mut rev_str = String::from("");

    //println!("String to reverse is {}", input);
    for c in input.chars().rev() {
        rev_str.push(c);
    }

    return rev_str;
}
