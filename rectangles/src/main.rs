
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

thread_local! {
    static mut rectangles :Vec<Rectangle> = Vec::new();
}

fn main() {
    
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {}", area(&rect1));

    println!("rect1 is {:?}", rect1);
    println!("The height of the rectangle is {}", rect1.height);
    println!("The width of the rectangle is {}", rect1.width);

    println!("The area of the rectangle is {}", rect1.area());

    rectangles.push(Rectangle{width: 20, height:10});

    println!("Rectangles vector {:?}", rectangles);
    
}

fn area(rect : &Rectangle) -> u32 { //Immutable borrow of struct Rectangle instance
    rect.width * rect.height
}