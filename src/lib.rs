// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.

use std::ptr;
use std::marker::PhantomData;
use std::mem::take;

pub struct LinkedList<T> {
    data: Option<T>,
    next: *mut LinkedList<T>,
    previous: *mut LinkedList<T>,
}

pub struct Cursor<'a, T> {
    pos: *mut LinkedList<T>,
    marker : PhantomData<&'a T>,
}

pub struct Iter<'a, T> {
    list: &'a LinkedList<T>,
    marker : PhantomData<&'a T>,
}

impl<T: Sized> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            data: None,
            next: ptr::null_mut(),
            previous: ptr::null_mut(),
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.previous.is_null()
    }

    pub unsafe fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut node = self;
        loop {
            if node.next.is_null() {
                break;
            } else {
                count += 1;
                node = node.next.as_ref().unwrap();
            }
        }
        count
    }

    /// Return a cursor positioned on the front element
    pub unsafe fn cursor_front(&self) -> Cursor<T> {
        Cursor {
            pos: self.previous.as_mut().unwrap(),
            marker : PhantomData,
        }
    }

    /// Return a cursor positioned on the back   
    pub unsafe fn cursor_back(&self) -> Cursor<T> {
        Cursor {
            pos: self.next.as_mut().unwrap(),
            marker : PhantomData,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { list: self }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<'a, T> Cursor<'a, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.pos.as_mut().unwrap().data.as_mut() }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.next()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe{self.pos.as_mut().unwrap().previous.as_mut().unwrap().data.as_mut()}
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        return match self.pos.is_null() {
            true => None,
            false => Some(self.pos.read().data.unwrap())
        }
    }

    pub fn insert_after(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn insert_before(&mut self, _element: T) {
        unimplemented!()
    }
}


impl<'a, T> Iterator for Cursor<'a, T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
      unsafe {
        self.pos.as_mut().unwrap().data.as_mut()
       } 
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
