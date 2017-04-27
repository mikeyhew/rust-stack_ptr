use std::borrow::{Borrow, BorrowMut};

/// A type that implements `Owned<'a, T>` has the lifetime `'a` and owns a value of type `T`, and has a known size at compilation time.
pub trait Owned<'a, T: 'a + ?Sized> {
}

impl<'a, T: 'a + ?Sized> Owned<'a, T> for Box<T> {
}

impl<'a, T: 'a> Owned<'a, T> for T {
}

#[macro_reexport]
pub mod stack_ptr;

pub use stack_ptr::StackPtr;

impl<'a, T: 'a> Owned<'a, T> for StackPtr<'a, T> {
}
