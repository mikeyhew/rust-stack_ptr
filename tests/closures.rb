#[macro_use]
extern crate stack_ptr;

fn execute_all<'a, I>(closures: I)
where I: IntoIterator<Item=StackPtr<'a, FnOnce()>> {
    for closure in closures {
        unimplemented!();
    }
}

#[test]
fn test_execute_all() {
    let mut callback1 = ||{};
    let mut callback1_lifetime = ();
    let callback1 = unsafe {
        let ptr = &mut callback1 as *mut FnOnce();
        mem::forget(callback1);
        StackPtr::from_mut(ptr, stack_ptr::constuct_mut_ref(ptr, &mut callback1_lifetime))
    };

    let mut callback2 = ||{};
    let mut callback2_lifetime = ();
    let callback2 = unsafe {
        let ptr = &mut callback2 as *mut FnOnce();
        mem::forget(callback2);
        StackPtr::from_mut(ptr, stack_ptr::constuct_mut_ref(ptr, &mut callback2_lifetime))
    };

    execute_all(vec![callback1, callback2]);
}
