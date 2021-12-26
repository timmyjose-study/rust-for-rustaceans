// unsafe functions have an implicit `unsafe` block.
unsafe fn do_something_sketchy() {
    let p: *const i32 = std::ptr::null();
    let p = p as *mut i32;
    *p = 100;
}

fn main() {
    unsafe {
        do_something_sketchy();
    }
}
