fn do_something() {
    let mut x = 42;
    unsafe {
        let mut p = &x as *const i32;
        println!("x through p = {}", *p);
        p = std::ptr::null(); // this is fine
    }

    unsafe {
        use std::ptr::NonNull;

        let mut p = NonNull::new_unchecked(&mut x as *mut i32);
        println!("x through p again = {}", *p.as_ptr());
        // this is not allowed
        //p = std::ptr::null();
    }
}

fn main() {
    do_something();
}
