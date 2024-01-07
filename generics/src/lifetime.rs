
/*
Lifetime annotations:
&i32 -> Reference
&'a i32 -> A reference with an explicit lifetime
&'a mut i32 -> A mutable reference with an explicit lifetime
*/

//This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field i.e. instance wouldn't exist if part 
//is dropped by Rust runtime.
struct ImportantExcerpt<'a> { //Structs that hold reference. 
    part: &'a str, //When structs hold reference, we need to provide lifetime annotations on them.
}

/*
The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

*/

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    //THIRD LIFETIME RULE APPLIES HERE
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//This tells Rust that the string slice returned from the function will live at least as long as lifetime 'a
pub fn longest<'a>(x :&'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn return_string_from_function() -> String {
    String::from("String from function to be used later")
}

// UNCOMMENTING THIS WOULD CAUSE COMPILATION FAILURE DUE TO THE WAY FUNCTION IS DEFINED
    // pub fn return_string_slice_from_function() -> &str {
//     String::from("String from function to be used later").as_str()
// }

//Static lifetime : Denotes that the affected reference can live for the entire duration of the program. 

/*
Generic Types, Traits bound and lifetimes all in one function
*/
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
