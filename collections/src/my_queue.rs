
pub trait Queue<T> {
    fn enqueue(&mut self, item : T) ->();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&mut self) -> Option<&T>;
}

pub struct MyQueue<T> {
    items : Vec<T>,
}

impl <T> MyQueue<T> {
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }
}

impl<T> Queue<T> for MyQueue<T> {
    fn enqueue(&mut self, item : T) ->() {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.items.len() > 0 {
            return Some(self.items.remove(0));
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<&T> {
        self.items.get(0)
    }
}