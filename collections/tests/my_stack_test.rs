use collections::my_stack::{MyStack, Stack};

#[test]
fn peek_on_empty_stack() {
    let mut my_stack : MyStack<u32> = MyStack::new();
    assert_eq!(my_stack.peek(), None);
}

#[test]
fn push_and_peek() {
    let mut my_stack : MyStack<u32> = MyStack::new();
    my_stack.push(2);
    my_stack.push(5);
    assert_eq!(my_stack.peek(), Some(5).as_ref());
}

#[test]
fn push_and_pop() {
    let mut my_stack : MyStack<u32> = MyStack::new();
    my_stack.push(2);
    my_stack.push(5);
    assert_eq!(my_stack.pop(), Some(5));
    assert_eq!(my_stack.peek(), Some(2).as_ref());
}

#[test]
fn pop_all() {
    let mut my_stack : MyStack<u32> = MyStack::new();
    my_stack.push(2);
    my_stack.push(5);
    my_stack.push(7);
    my_stack.push(9);

    my_stack.pop();
    my_stack.pop();
    my_stack.pop();
    my_stack.pop();

    assert_eq!(my_stack.peek(), None);
}

#[test]
fn count() {
    let mut my_stack : MyStack<u32> = MyStack::new();

    assert_eq!(my_stack.count(), 0);

    my_stack.push(2);
    my_stack.push(5);
    my_stack.push(7);
    my_stack.push(9);

    assert_eq!(my_stack.count(), 4);
}