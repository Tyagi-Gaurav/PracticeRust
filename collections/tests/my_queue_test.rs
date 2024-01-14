use collections::my_queue::{MyQueue, Queue};

#[test]
fn peek_on_empty_queue() {
    let mut my_queue : MyQueue<u32> = MyQueue::new();
    assert_eq!(my_queue.peek(), None);
}

#[test]
fn enqueue_and_peek() {
    let mut my_queue : MyQueue<u32> = MyQueue::new();
    my_queue.enqueue(2);
    my_queue.enqueue(5);
    assert_eq!(my_queue.peek(), Some(2).as_ref());
}

#[test]
fn enqueue_and_dequeue() {
    let mut my_queue : MyQueue<u32> = MyQueue::new();
    my_queue.enqueue(2);
    my_queue.enqueue(5);
    assert_eq!(my_queue.dequeue(), Some(2));
    assert_eq!(my_queue.peek(), Some(5).as_ref());
}

#[test]
fn dequeue_all() {
    let mut my_queue : MyQueue<u32> = MyQueue::new();
    my_queue.enqueue(2);
    my_queue.enqueue(5);
    my_queue.enqueue(7);
    my_queue.enqueue(9);

    my_queue.dequeue();
    my_queue.dequeue();
    my_queue.dequeue();
    my_queue.dequeue();

    assert_eq!(my_queue.peek(), None);
}