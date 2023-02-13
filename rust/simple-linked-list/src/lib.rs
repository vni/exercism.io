use std::iter::FromIterator;

type NodePtr<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: NodePtr<T>,
}

pub struct SimpleLinkedList<T> {
    head: NodePtr<T>,

}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut node = &self.head;
        while let Some(n) = node {
            node = &n.next;
            count += 1;
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let head = self.head.take();
        let new_head = Some(Box::new(Node {
            data: element,
            next: head,
        }));
        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut h) = self.head.take() {
            let new_head = h.next.take();
            self.head = new_head;
            return Some(h.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        println!("Hello world!");
        None//unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        SimpleLinkedList{head: None}//unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        SimpleLinkedList{head:None}//unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        vec![]//unimplemented!()
    }
}
