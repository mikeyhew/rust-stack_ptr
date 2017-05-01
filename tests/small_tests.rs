#[macro_use]
extern crate stack_ptr;

use stack_ptr::StackPtr;

#[test]
fn test_basic() {
    declare_stackptr!{
        let slice: StackPtr<[i32]> = StackPtr::new([1,2,3,4,5]);
    }

    assert_eq!(&*slice, &[1,2,3,4,5]);
}
