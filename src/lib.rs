/*!
Provides `StackPtr`, an owned pointer to stack-allocated data. This lets you cast a value to an unsized type (e.g. trait object) while maintaining ownership and without doing a heap allocation with `Box`.

# Example Usage

```
#[macro_use]
extern crate stack_ptr;
use stack_ptr::StackPtr;

fn main() {
    declare_stackptr! {
        let callback: StackPtr<Fn()> = StackPtr::new(||{});
    }

    // this example should use call_once(). call() isn't a very motivating example to have an owned reference.
    callback();
}
```
*/
#![cfg_attr(feature = "nightly", feature(unsize, coerce_unsized))]

extern crate stable_deref_trait;

mod impls;
pub mod iter;

use std::marker::PhantomData;
use std::{ptr, mem};

/// An owned pointer type to stack-allocated data. See the module-level documentation for further details.
pub struct StackPtr<'a, T: 'a + ?Sized> {
    ptr: &'a mut T,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a + ?Sized> StackPtr<'a, T> {
    /// Constructs a new `StackPtr` from an `&mut` reference. The `StackPtr` will assume ownership of the pointed-to value, so make sure you call `std::mem::forget` on the value to avoid double-drop. The `declare_stackptr!` macro does this all safely for you.
    pub unsafe fn from_mut(ptr: &'a mut T) -> StackPtr<'a, T> {
        StackPtr {
            ptr: ptr,
            _marker: PhantomData,
        }
    }

    /// Consumes a `StackPtr` without dropping it, and returns a `&mut` reference to the data. Useful for doing a coercion on the reference and reconstructing a new `StackPtr` with `from_mut`.
    pub fn into_mut(sp: StackPtr<'a, T>) -> &'a mut T {
        unsafe {
            let ptr = sp.ptr as *mut T;
            mem::forget(sp);
            &mut *ptr
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

#[doc(hidden)]
#[inline(always)]
pub unsafe fn construct_mut_ref<'a, T: ?Sized>(ptr: *mut T, _lifetime_marker: &'a mut ()) -> &'a mut T {
    &mut *ptr
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_stackptr_variable {
    (no, $name:ident, $expr:expr) => {
        let $name = $expr;
    };
    (yes, $name:ident, $expr:expr) => {
        let mut $name = $expr;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_stackptr {
    ($mutable:ident, $name:ident, $ty:ty, $expr:expr) => {
        let mut _value = $expr;
        let mut _lifetime_marker = ();
        __declare_stackptr_variable!($mutable, $name, unsafe {
            let ptr = &mut _value as *mut $ty;
            ::std::mem::forget(_value);
            StackPtr::from_mut($crate::construct_mut_ref(ptr, &mut _lifetime_marker))
        });
    };
}

/// Safely declare a `StackPtr` with the appropriate lifetime at this point on the stack.
#[macro_export]
macro_rules! declare_stackptr {
    (let $name:ident: StackPtr<$ty:ty> = StackPtr::new($expr:expr);) => {
        __declare_stackptr!(no, $name, $ty, $expr)
    };
    (let $name:ident = StackPtr::new($expr:expr);) => {
        __declare_stackptr!(no, $name, _, $expr)
    };
    (let mut $name:ident: StackPtr<$ty:ty> = StackPtr::new($expr:expr);) => {
        __declare_stackptr!(yes, $name, $ty, $expr)
    };
    (let mut $name:ident = StackPtr::new($expr:expr);) => {
        __declare_stackptr!(yes, $name, _, $expr)
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
