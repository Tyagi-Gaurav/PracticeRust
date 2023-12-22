use generics::summary::{Summary, Tweet, NewsArticle};
use generics::lifetime::*;

struct Point<T, U> {
    x : T,
    y : U,
}

impl <T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x //defined a method named x on Point<T> that returns a reference to the data in the field x.
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 25];
    let char_list = vec!['a', 'u', 'i', 'o', 'e'];

    println!("Largest number is {}", largest_generic(&number_list));
    println!("Largest number is {}", largest_generic(&char_list));

    let both_integer = Point {x: 5, y:10};
    let both_float = Point {x: 5.0, y:10.0};
    let float_and_int = Point {x: 5.0, y:10};

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let sff = return_string_from_function();
    println!("String from function is {}", sff);
    let sff_str : &str = sff.as_str();
    println!("String slice from function is {}", sff_str);
    
    // UNCOMMENTING THIS WOULD CAUSE COMPILATION FAILURE DUE TO THE WAY FUNCTION IS DEFINED IN LIB
    //
    // let ssff = return_string_slice_from_function();
    // println!("String from function is {}", ssff);
}

fn largest(number_list : &[i32]) -> &i32 {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }    
    }

    return largest;
}

fn largest_generic<T: std::cmp::PartialOrd>(list : &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }    
    }

    return largest;
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}