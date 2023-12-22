use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") //Default implementation of method in
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
*/

/*
Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary).

pub fn notify(item1: &impl Summary, item2: &impl Summary);
*/
pub fn notify(item : &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyWithDisplay(item : &(impl Summary + Display)) {
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

