//! Provides `StackPtr`, an owned pointer to stack-allocated data. This is useful for casting a value to an unsized type (e.g. trait object) while maintaining ownership, without doing a heap allocation with `Box`.
#![cfg_attr(feature = "nightly", feature(unsize, coerce_unsized))]
use std::marker::PhantomData;
use std::{ptr, mem};
use std::ops::{Deref, DerefMut};

/// An owned pointer type to stack-allocated data. See the module-level documentation for further details.
pub struct StackPtr<'a, T: 'a + ?Sized> {
    ptr: *mut T,
    lifetime: PhantomData<&'a mut ()>,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a + ?Sized> StackPtr<'a, T> {
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
            lifetime: lifetime,
            _marker: PhantomData,
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
        unsafe {
            &*self.ptr
        }
    }
}

impl<'a, T: ?Sized> DerefMut for StackPtr<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.ptr
        }
    }
}

unsafe impl<'a, T: 'a + Send + ?Sized> Send for StackPtr<'a, T> {}
unsafe impl<'a, T: 'a + Sync + ?Sized> Sync for StackPtr<'a, T> {}

#[doc(hidden)]
pub fn lifetime_of<'a, T>(_ref: &'a mut T) -> PhantomData<&'a mut ()> {
    PhantomData
}

/// Safely declare a `StackPtr` with the appropriate lifetime at this point on the stack.
///
/// ```
/// stackptr! {
///     let foo: StackPtr<> = StackPtr::new()
/// }
/// ```
#[macro_export]
macro_rules! stack_ptr {
        let mut _value = $expr;
        let mut _lifetime_marker = ();
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
        let (ptr, lifetime) = $crate::StackPtr::into_raw_parts($sp);
        unsafe {
            $crate::StackPtr::from_raw_parts(ptr as *mut $ty, lifetime)
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
