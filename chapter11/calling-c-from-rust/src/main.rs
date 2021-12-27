#![feature(c_size_t)]

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_size_t};

fn calling_c_stdlib() {
    extern "C" {
        #[link_name = "printf"]
        fn cprintf(fmt: *const c_char, args: ...) -> c_int;
        fn strlen(s: *const c_char) -> c_size_t;
    }

    let fmt = CString::new("%s has length %lu\n").unwrap();
    let s = CString::new("hello, world").unwrap();
    unsafe {
        let len = cprintf(fmt.as_ptr(), s.as_ptr(), strlen(s.as_ptr()));
        assert_eq!(len, 27); // including the null terminator
    }
}

// this, for some reason, only seems to work with `DYLD_LIBRARY_PATH` set while invoking `cargo run`
fn calling_custom_c_dynamic_library() {
    #[link(name = "calc1", kind = "dylib")]
    extern "C" {
        fn add(x: c_int, y: c_int) -> c_int;
        fn sub(x: c_int, y: c_int) -> c_int;
        fn mul(x: c_int, y: c_int) -> c_int;
        fn div(x: c_int, y: c_int) -> c_int;
    }

    let x = 42;
    let y = 12;

    unsafe {
        println!("{} + {} = {}", x, y, add(x, y));
        println!("{} - {} = {}", x, y, sub(x, y));
        println!("{} * {} = {}", x, y, mul(x, y));
        println!("{} / {} = {}", x, y, div(x, y));
    }
}

fn calling_custom_c_static_library() {
    #[link(name = "calc2", kind = "static")]
    extern "C" {
        fn add(x: c_int, y: c_int) -> c_int;
        fn sub(x: c_int, y: c_int) -> c_int;
        fn mul(x: c_int, y: c_int) -> c_int;
        fn div(x: c_int, y: c_int) -> c_int;
    }

    let x = 42;
    let y = 12;

    unsafe {
        println!("{} + {} = {}", x, y, add(x, y));
        println!("{} - {} = {}", x, y, sub(x, y));
        println!("{} * {} = {}", x, y, mul(x, y));
        println!("{} / {} = {}", x, y, div(x, y));
    }
}

fn main() {
    calling_c_stdlib();
    println!();
    calling_custom_c_static_library();
    println!();
    calling_custom_c_dynamic_library();
}
