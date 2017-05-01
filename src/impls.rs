use std::fmt::{Debug, Formatter};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use super::StackPtr;

impl<'a, T: ?Sized> Debug for StackPtr<'a, T> where T: Debug {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        self.deref().fmt(formatter)
    }
}

impl<'a, T: ?Sized> Deref for StackPtr<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<'a, T: ?Sized> DerefMut for StackPtr<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.ptr
    }
}

impl<'a, T: ?Sized> AsRef<T> for StackPtr<'a, T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<'a, T: ?Sized> AsMut<T> for StackPtr<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

unsafe impl<'a, T: 'a + Send + ?Sized> Send for StackPtr<'a, T> {}
unsafe impl<'a, T: 'a + Sync + ?Sized> Sync for StackPtr<'a, T> {}
