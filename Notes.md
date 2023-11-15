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
* The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

## References and Borrowing
* A reference is like a pointer in that it’s an address we can follow to access the data stored at that address.
*  The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`.
* When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.
* Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

## Mutable Referennces
* Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
* The benefit of having this restriction is that Rust can prevent data races at compile time. 
* A data race is similar to a race condition and happens when these three behaviors occur:
*  We also cannot have a mutable reference while we have an immutable one to the same value.

## Slices
* A slice is a kind of reference, so it does not have ownership.
* A string slice is a reference to part of a String.
* If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of deref coercions