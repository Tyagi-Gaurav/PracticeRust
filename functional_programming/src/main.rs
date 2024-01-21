
fn main() {
    /*
    If you want to force the closure to take ownership of the values it uses in the environment even though the body 
    of the closure doesn’t strictly need ownership, you can use the move keyword before the parameter list.


    use std::thread;

    fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

     */
    let example_closure = |x| x;
}

/*
The way a closure captures and handles values from the environment affects which traits the closure implements
*/