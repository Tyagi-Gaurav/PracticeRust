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

fn main() {
    
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let mut rectangles : Vec<Rectangle> = vec![];

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

#[cfg(test)] //annotation on the tests module tells Rust to compile and run the test code only when you run cargo test
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}