
//This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> { //Structs that hold reference
    part: &'a str,
}

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