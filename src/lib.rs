//! Provides `StackPtr`, an owned pointer to stack-allocated data. Create one
//! using the provided `stack_ptr!` macro:
//!
//! ```
//! stack_ptr! {
//!     let slice: StackPtr<[_]> = StackPtr::new([1,2,3,4,5]);
//! }
//! ```
use std::marker::PhantomData;
use std::cell::Cell;
use std::ptr;
use std::ops::{Deref, DerefMut};

/// An owning pointer to stack-allocated data. Similar to `Box`, except `Box` is heap-allocated.
pub struct StackPtr<'a, T: 'a + ?Sized> {
    ptr: *mut T,
    _marker: PhantomData<T>,
    _lifetime: PhantomData<&'a mut Cell<()>>
}

impl<'a, T: 'a + ?Sized> StackPtr<'a, T> {
    /// `ptr` must be a pointer to forgotten data, whose lifetime is at least as long as `lifetime`'s lifetime. Better to just use the `stack_ptr!` macro.
    pub unsafe fn new(ptr: *mut T, _lifetime: &'a mut ()) -> StackPtr<'a, T>{
        StackPtr {
            ptr: ptr,
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// an implementation of std::borrow::Borrow, where the returned reference has the same lifetime as `self`.
    pub fn borrow(&self) -> &'a T {
        unsafe {
            &*self.ptr
        }
    }

    /// an implementation of std::borrow::BorrowMut, where the returned reference has the same lifetime as `self`.
    pub fn borrow_mut(&mut self) -> &'a mut T {
        unsafe {
            &mut *self.ptr
        }
    }
}

impl<'a, T: ?Sized> Drop for StackPtr<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr);
        }
    }
}

impl<'a, T: ?Sized> ::std::fmt::Debug for StackPtr<'a, T> where T: ::std::fmt::Debug {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.deref().fmt(formatter)
    }
}

impl<'a, T: ?Sized> Deref for StackPtr<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.borrow()
    }
}

impl<'a, T: ?Sized> DerefMut for StackPtr<'a, T> {

    fn deref_mut(&mut self) -> &mut T {
        self.borrow_mut()
    }
}

unsafe impl<'a, T: 'a + Send + ?Sized> Send for StackPtr<'a, T> {}
unsafe impl<'a, T: 'a + Sync + ?Sized> Sync for StackPtr<'a, T> {}

/// Safely declares a StackPtr<$ty> with an appropriate lifetime to the data contained in $expr.
#[macro_export]
macro_rules! stack_ptr {
    (let $name:ident: StackPtr<$ty:ty> = StackPtr::new($expr:expr);) => {
        let mut _value = $expr;
        let mut _lifetime_marker = ();
        let $name = unsafe {
            let ptr = &mut _value as *mut $ty;
            ::std::mem::forget(_value);
            StackPtr::new(ptr, &mut _lifetime_marker)
        };
    };
}
