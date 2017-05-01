use std::marker::PhantomData;
use std::slice;
use std::{ptr};

use super::StackPtr;

pub struct SliceIntoIter<'a, T: 'a> {
    slice_iter: slice::IterMut<'a, T>,
    _marker: PhantomData<[T]>,
}

impl<'a, T> Drop for SliceIntoIter<'a, T> {
    fn drop(&mut self) {
        for ptr in &mut self.slice_iter {
            unsafe {
                ptr::drop_in_place(ptr)
            }
        }
    }
}

impl<'a, T> Iterator for SliceIntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.slice_iter.next().map(|ptr| {
            unsafe {
                ptr::read(ptr)
            }
        })
    }
}

impl<'a, T> IntoIterator for StackPtr<'a, [T]> {
    type Item = T;
    type IntoIter = SliceIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SliceIntoIter {
            slice_iter: StackPtr::into_mut(self).into_iter(),
            _marker: PhantomData
        }
    }
}
