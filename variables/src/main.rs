fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /*
    Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences
    between constants and variables.
     */

    //Constants are always immutable. Eg.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //SHadowing of variables
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    /*
    Because we’re effectively creating a new variable when we use the let keyword again,
    we can change the type of the value but reuse the same name.
     */
    let spaces = "   "; //String type
    let spaces = spaces.len(); //Number type

    /*
    Tuples: A tuple is a general way of grouping together a number of values with a variety of types into one compound type.

     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; //This is called destructuring

    println!("The value of y is: {y}");

    /*
    Arrays
     */
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //Here i32 is the type of each element.

    let a = [3; 5]; //Initializes array to contain the same value 5 times.

    another_function(2); //Calling a function is an expression

    //Calling a macro is an expression.
    //A new scope block created with curly brackets is an expression,
    let y = {
        let x = 3;
        x + 1 //Expressions do not include ending semicolons
    };
    println!("The value of y is: {y}");

    println!("{}", five());

    let mut counter = 0;
    while counter != 9 {
        print!("{} ", fizz_buzz(counter));
        counter += 1;
    }
    println!();

    for number in (1..4) {
        print!("{} ", fizz_buzz(number));
    }

    /*
       Ownership Principles
       * Each value in Rust has an owner.
       * There can only be one owner at a time.
       * When the owner goes out of scope, the value will be dropped
    */

    /*
    Just as variables are immutable by default, so are references.
    We’re not allowed to modify something we have a reference to.
     */

    //Changing via a reference
    let mut s = String::from("hello");
    change(&mut s);
    //We also cannot have a mutable reference while we have an immutable one to the same value.

    println!();
    println!("{s}");
    let input = &String::from("To be or not to be");
    let first_word = first_word(input);
    println!("First word: {first_word}");
}

fn first_word(s : &String) -> &str { //&str is a string slice that refers to a specific sequence of characters in a string
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn fizz_buzz(num: i32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        return String::from("Fizz Buzz");
    } else if num % 3 == 0 {
        return String::from("Fizz");
    } else if num % 5 == 0 {
        return String::from("Buzz");
    }
    String::from("")
}

fn another_function(x: i32) {
    println!("Another function called with {x}.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 //No-semicolon needed as its an expression and not a statement.
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* 
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    //This works without any problems. Ownership is moved out, and 
    //nothing is deallocated.
    s
}
