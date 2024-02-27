use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::<T>::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer)
        };
        self.buffer.push_back(_element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            return Err(Error::EmptyBuffer)
        };
        Ok(self.buffer.pop_front().unwrap())
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(_element)
    }

    fn is_full(&self) -> bool {
        self.buffer.len() == self.buffer.capacity()
    }
}
