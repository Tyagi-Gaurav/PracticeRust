
pub trait Stack<T> {
    fn push(&mut self, item : T) ->();
    fn pop(&mut self) -> Option<T>;
    fn peek(&mut self) -> Option<&T>;
    fn count(&mut self) -> usize;
}

pub struct MyStack<T> {
    items : Vec<T>,
}

impl <T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }
}

impl<T> Stack<T> for MyStack<T> {
    fn push(&mut self, item : T) ->() {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(last_element) = self.items.last() {
            Some(self.items.remove(self.items.len() - 1))
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<&T> {
        self.items.last()
    }

    fn count(&mut self) -> usize {
        self.items.len()
    }
}