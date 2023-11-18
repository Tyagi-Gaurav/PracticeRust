
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8), //4 numeric components
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1, //match arms
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/*
There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, 
we’re also able to define methods on enums.
*/

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let home2 = IpAddrKind2::V4(127, 0, 0, 1);

    let some_number = Some(5);
    let v = value_in_cents(Coin::Dime);
    println!("Value in cents: {v}");

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //Counting all non-quarters
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

}
