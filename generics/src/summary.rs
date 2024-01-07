use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") //Default implementation of method in types.
    }
}

pub struct NewsArticle {
    pub headline : String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username : String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }   
}
/*
we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, 
because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate.

We can implement a trait on a type only if at least one of the trait or the type is local to our crate.
We can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality, 
because the type Tweet is local to our current crate.
*/

/*
Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary).

pub fn notify(item1: &impl Summary, item2: &impl Summary);
*/
pub fn notify(item : &impl Summary) { //Look below for better syntax.
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyWithDisplay(item : &(impl Summary + Display)) { //or pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type of the value passed as an 
argument for item1 and item2 must be the same.

pub fn notify<T: Summary>(item1: &T, item2: &T);
*/
pub fn notify2<T : Summary>(item : &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
Using where close for clearer trait bounds
*/
fn some_function<T, U>(t : &T, u : &U) -> i32 
where 
    T: Display + Clone,
    U: Clone + std::fmt::Debug, {
        0
}

/*
Conditionally implementing a trait
*/
struct Pair<T> {
    x : T,
    y : T,
}

impl <T> Pair<T> {
    fn new(x : T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T : Display + PartialOrd> Pair<T> { //Only applies to types that implement Display and PartialOrd
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}