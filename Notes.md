## Commands
* `cargo build`
* `cargo run`
* `cargo check`
* `cargo new <project_name>`
* `cargo build --release`
* `cargo update` To ignore .lock file and figure out the dependencies again.
* `cargo doc --open` : To generate documentation and open in browser

## Programs
* Hello world (/)
* Fizz Buzz (/)
* Fibonacci
* Return the first word found in a string

## Memory & Allocation
* In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. 
* This is why string literals are fast and efficient.
* If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
* The mechanics of passing a value to a function are similar to those when assigning a value to a variable. 
* Passing a variable to a function will move or copy, just as assignment does. 
* The ownership of a variable follows the same pattern every time: 
    * Assigning a value to another variable moves it. 
    * When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    * If you don't want a function to take ownership of parameters, then pass references (`&`) to the function.
    * When returning references from function
        * When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters
        * If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
            * However, this would be a dangling reference because the value will go out of scope at the end of the function.
    * Borrowing could either be
        * Borrowing immutably
        * Borrowing mutably
        * Taking ownership

## References and Borrowing
* A reference is like a pointer in that it’s an address we can follow to access the data stored at that address.
* The opposite of referencing `&` is dereferencing, which is accomplished with the dereference operator, `*`.
* When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.
* Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

## Mutable References
* Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
* The benefit of having this restriction is that Rust can prevent data races at compile time. 
* A data race is similar to a race condition and happens when these three behaviors occur.
* We also cannot have a mutable reference while we have an immutable one to the same value.

## Slices
* A slice is a kind of reference, so it does not have ownership.
* A string slice is a reference to part of a String.
* If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of deref coercions30

## Packages & Modules
* Packages; Lets you share crates.
    * A bundle of one or more crates.
    * Can contain as many binary crates as possible.
    * But atmost one library crate.
* Crates: A tree of modules that produces a library or executable
    * A binary crate: An executable is produced and should have a `main()` function
    * library crate: They define functionality that can be shared with multiple projects. They don't have `main()` function.
    * Crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.
* Modules and use: Lets you control the organization, scope and privacy of paths
* Paths: A way of naming an item, such as struct, function or module.
* Cargo
    * Cargo follows a convention that `src/main.rs` is the crate root of a binary crate
    * Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root

## Difference between `self`, `&self` & `&Self`
```
impl MyTrait for i32 {
    fn to_larger(self) ->i64 {
        // This function takes ownership of self, meaning self is effectively destroyed when this function runs.
    }

    fn add_one(&mut self) {
        // This function takes a mutable reference, it can modify itself, but will continue to live after this function
    }
    fn new() -> Self {
        // Self means the current type, which is i32. This function returns something that is an i32.
    }
}
```

## Lifetimes
```
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```
* The above fails because `r` refers to `x` that has a shorter lifetime. 
* Lifetime syntax
    * `&i32`: A reference
    * `&'a i32`: A reference with an explicit lifetime
    * `&'a mut i32`: A mutable reference with an explicit lifetime
    * `&'static str` : A static lifetime
    
* The patterns programmed into Rust’s analysis of references are called the lifetime elision rules
    * Eg. The following code works without specifying the lifetime parameters. 
    ```
        fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
        }
    ```
    * 
* The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. 
    * It assigns a lifetime parameter to each parameter that’s a reference.
        * A function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` 
        * A function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on
    * If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32.`
    * If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters.
