#![allow(unused)]
// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
#![allow(unused)]
use std::{borrow::BorrowMut, boxed::Box};

mod pre_implemented;


#[derive(Debug, Clone)]
pub struct Node<T>(T);

#[derive(Clone)]
pub struct LinkedList<T> {
    pub list: Vec<Node<T>>,
    idx: usize
}

#[derive(Debug)]
pub struct Cursor<'a, T> {
    pub nodes: &'a mut Vec<Node<T>>,
    idx: usize
}

#[derive(Debug)]
pub struct Iter<'a, T>(&'a Vec<Node<T>>, usize, usize);

impl<'a, T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            list: vec![],
            idx: 0
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&'a mut self) -> Cursor<'_, T> {
        Cursor {
            nodes: &mut self.list,
            idx: 0
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let last = self.len().checked_sub(1).unwrap_or_default();
        Cursor {
            nodes: &mut self.list,
            idx: last
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(&self.list, 0, self.len())
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.nodes.is_empty() {
            return None
        }
        Some(&mut self.nodes[self.idx].0)
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if self.nodes.is_empty() {
            return None
        }
        let next_idx = self.next_idx();

        if next_idx == self.idx {
            None
        } else {
            self.idx = next_idx;
            Some(&mut self.nodes[next_idx].0)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.nodes.is_empty() {
            return None
        }
        let next_idx = self.prev_idx();

        if next_idx == self.idx {
            None
        } else {
            self.idx = next_idx;
            Some(&mut self.nodes[next_idx].0)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        // let item = 
        self.nodes.remove(self.idx);
        self.idx = self.idx.min(self.last());
        // Some(item.0)
        None
    }

    pub fn insert_after(&mut self, _element: T) {
        if self.idx == self.last() {
            self.nodes.push(Node(_element));
        } else {
            self.nodes.insert(self.next_idx(), Node(_element));
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        self.nodes.insert(self.idx, Node(_element));
        self.idx = self.next_idx();
    }

    fn last(&self) -> usize {
        self.nodes.len().checked_sub(1).unwrap_or_default()
    }

    fn next_idx(&self) -> usize {
        (self.idx + 1).min(self.last())
    }

    fn prev_idx(&self) -> usize {
        self.idx.checked_sub(1).unwrap_or_default()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.1 == self.2 {
            return None
        }

        let item = Some(&self.0[self.1].0);
        self.1 += 1;

        item
    }
}
