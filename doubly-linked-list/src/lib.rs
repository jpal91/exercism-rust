#![allow(unused)]
#![cfg(feature = "ERROR")]
// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
use std::boxed::Box;

// mod pre_implemented;


struct Node<'a, T> {
    val: Option<T>,
    next: Option<Box<&'a Node<'a, T>>>,
    prev: Option<Box<&'a Node<'a, T>>>,
}

pub struct LinkedList<'a, T> {
    root: Node<'a, T>,
    last: Node<'a, T>
}

pub struct Cursor<'a, 'b, T>(&'b mut Node<'a, T>);

pub struct Iter<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, 'b, T: Clone> LinkedList<'a, T> {
    pub fn new() -> Self {
        let mut first: Node<T> = Node { val: None, next: None, prev: None };
        let mut last: Node<T> = Node { val: None, next: None, prev: None };

        first.next = Some(Box::new(&last));
        last.prev = Some(Box::new(&first));

        Self {
            root: first,
            last
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&'a mut self) -> Cursor<'_, '_, T> {
        Cursor(&mut self.root)
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, '_, T> {
        todo!()
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, '_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        todo!()
    }

    pub fn insert_after(&mut self, _element: T) {
        todo!()
    }

    pub fn insert_before(&mut self, _element: T) {
        todo!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        todo!()
    }
}
