fn main() {
    println!("Hello, world!");

    //by default, variables are immutable
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    let a = "abc";
    let b = "def";


    if &a == &b {
        println!("Equal");
    } else {
        println!("Not Equal"); //Will print Not-Equal
    }
}