use std::marker::PhantomData;
use std::cell::Cell;
use std::{ptr, mem};

pub struct StackPtr<'a, T: 'a + ?Sized> {
    ptr: *mut T,
    _marker: PhantomData<T>,
    _lifetime: PhantomData<&'a mut Cell<()>>
}

impl<'a, T: 'a + ?Sized> StackPtr<'a, T> {
    /// Creates a StackPtr
    pub unsafe fn new(ptr: *mut T, lifetime: &'a mut ()) -> StackPtr<'a, T>{
        StackPtr {
            ptr: ptr,
            _marker: PhantomData,
            _lifetime: PhantomData,
        }
    }

    pub fn borrow(&self) -> &'a T {
        unsafe {
            &*self.ptr
        }
    }

    pub fn borrow_mut(&mut self) -> &'a T {
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

// impl <'a, Borrowed: 'a + ?Sized> ScopedBorrow<'a, Borrowed> for StackPtr<'a, Borrowed> {
//     fn scoped_borrow(&'a self) -> &'a Borrowed {
//         self.borrow_mut()
//     }
// }
//
// impl <'a, Borrowed: 'a + ?Sized> ScopedBorrowMut<'a, Borrowed> for StackPtr<'a, Borrowed> {
//     fn scoped_borrow_mut(&'a mut self) -> &'a Borrowed {
//         self.borrow_mut()
//     }
// }

pub fn sound() {
    stack_ptr! {
        let slice: StackPtr<[_]> = StackPtr::new([1,2,3]);
    }

    let slice2 = slice;
    let slice3 = slice;
}

// pub fn dangling() -> StackPtr<'static, [i32]> {
//     stack_ptr! {
//         let bar: StackPtr<[_]> = StackPtr::new([1,2,3,4,5]);
//     }
//
//     bar
// }
