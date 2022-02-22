// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.

use std::ptr;
use std::marker::PhantomData;

pub struct LinkedList<T> {
    data: Option<Box<T>>,
    next: *mut LinkedList<T>,
    previous: *mut LinkedList<T>,
    len: usize,
}

pub struct Cursor<'a, T> {
    pos: *mut LinkedList<T>,
    _marker : PhantomData<&'a T>
}

pub struct Iter<'a, T>{
    list : &'a LinkedList<T>,
    _marker : PhantomData<&'a T>,
}

impl<T: Sized> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            data: None,
            next: ptr::null_mut(),
            previous: ptr::null_mut(),
            len: usize::default(),
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        return match self.data {
            None => true,
            Some(_) => false,
        };
    }

    pub fn len(&self) -> usize {
        return if !Self::is_empty(self) { 0 } else { self.len };
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&self) -> Cursor<'_, T> {
        Cursor { pos: self.previous,_marker : PhantomData }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&self) -> Cursor<T> {
        Cursor { pos: self.next ,_marker : PhantomData}
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter{
            list : self,
            _marker : PhantomData
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn insert_after(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn insert_before(&mut self, _element: T) {
        unimplemented!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}
