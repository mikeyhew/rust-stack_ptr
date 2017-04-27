//! Provides `StackPtr`, an owned pointer to stack-allocated data. Create one
//! using the provided `stack_ptr!` macro:
//!
//! ```
//! stack_ptr! {
//!     let slice: StackPtr<[_]> = StackPtr::new([1,2,3,4,5]);
//! }
//! ```
#![cfg_attr(feature = "nightly", feature(unsize, coerce_unsized))]
use std::marker::PhantomData;
use std::{ptr, mem};
use std::ops::{Deref, DerefMut};

/// An owning pointer to stack-allocated data. Similar to `Box`, except `Box` is heap-allocated.
pub struct StackPtr<'a, T: 'a + ?Sized> {
    ptr: *mut T,
    _marker: PhantomData<T>,
    lifetime: PhantomData<&'a mut ()>
}

impl<'a, T: 'a + ?Sized> StackPtr<'a, T> {
    /// `ptr` must be a pointer to forgotten data on the stack, whose lifetime is at least as long as `lifetime`'s lifetime. Better to just use the `stack_ptr!` macro.
    pub unsafe fn new(ptr: *mut T, _lifetime: &'a mut ()) -> StackPtr<'a, T> {
        StackPtr {
            ptr: ptr,
            lifetime: PhantomData,
            _marker: PhantomData,
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

    /// Consumes a `StackPtr` without running its destructor, and returns a `*mut` pointer to the data, and a `std::marker::PhantomData` representing the lifetime of the `StackPtr`. Useful for doing a conversion on the pointer and reconstructing a `StackPtr` with `from_raw_parts`.
    pub fn into_raw_parts(sp: StackPtr<'a, T>) -> (*mut T, PhantomData<&'a mut ()>) {
        let ret = (sp.ptr, sp.lifetime);
        mem::forget(sp);
        ret
    }

    /// Constructs a new `StackPtr` from its raw parts. Usually called on the result of `into_raw_parts`.
    pub unsafe fn from_raw_parts(ptr: *mut T, lifetime: PhantomData<&'a mut ()>) -> StackPtr<'a, T> {
        StackPtr {
            ptr: ptr,
            _marker: PhantomData,
            lifetime: lifetime
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

/// An implementation of `std::ops::CoerceUnsized` on stable rust. On nightly, you can convert a `StackPtr<T>` into a `StackPtr<U>` if `T` implements `U`, with `let sp = sp as StackPtr<U>;`, but this requires the unstable `CoerceUnsized` trait. On stable you can do `let sp = coerce_stackptr!(sp, U);`.
#[macro_export]
macro_rules! coerce_stackptr {
    ($sp:expr, $ty:ty) => {{
        let (ptr, lifetime) = StackPtr::into_raw_parts($sp);
        unsafe {
            StackPtr::from_raw_parts(ptr as *mut $ty, lifetime)
        }
    }};
}

#[cfg(feature="nightly")]
mod nightly {
    use super::StackPtr;
    use std::ops::CoerceUnsized;
    use std::marker::Unsize;

    impl<'a, T, U> CoerceUnsized<StackPtr<'a, U>> for StackPtr<'a, T> where T: Unsize<U> + ?Sized, U: ?Sized {}
}
