use time_macros::date;
use time_macros::time;
use time::PrimitiveDateTime as DateTime;

fn main() {
    // greatest_of_three_numbers(4, 2, 5);
    // reverse_first_last_name();
    // display_first_last_items_from_list();
    // simple_programs::reverse_string();
    // simple_programs::gigasecond_after_start_time(DateTime::new(date!(2019-01-01), time!(0:00)));
}

fn reverse_first_last_name() {
    let mut full_name = "Jack Tommy";

    println!("Full name is {} ", full_name);
    print!("Full name in reverse is ");
    for word in full_name.split_whitespace().rev() {
        print!("{} ", word);
    }
}

fn greatest_of_three_numbers(a: i32, b: i32, c: i32) {
    let mut max = a;
    if b > max {
        max = b;
    }

    if c > max {
        max = c;
    }

    println!("Greatest of 3 numbers is {max}");
}

fn display_first_last_items_from_list() {
    let mut v = Vec::new();

    v.push(11);
    v.push(22);
    v.push(33);
    v.push(55);
    v.push(7);

    println!("First Item from List is {} ", v[0]);
    println!("last Item from List is {} ", v[v.len() - 1]);
}
