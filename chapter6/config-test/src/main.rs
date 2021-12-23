#![feature(bench_black_box)]

use config_test::Foo;

//  undefined behaviour - use `cargo miri run` to catch this at
//  compile time.
fn miri_unsafe_aliasing() {
    let mut x = 42;
    let p: *mut i32 = &mut x;
    let (x1, x2) = unsafe { (&mut *p, &mut *p) };
    println!("{} {}", x1, x2);
}

fn no_black_box() {
    let mut vs = Vec::with_capacity(1000);
    let start = std::time::Instant::now();
    for i in 0..10_000_000 {
        vs.push(i);
    }
    println!("Without black box, {:?}", start.elapsed());
}

fn with_black_box() {
    let mut vs = Vec::with_capacity(1000);
    let start = std::time::Instant::now();
    for i in 0..10_000_000 {
        std::hint::black_box(vs.as_ptr());
        vs.push(i);
        std::hint::black_box(vs.as_ptr());
    }
    println!("With black box, {:?}", start.elapsed());
}

fn main() {
    let foo = Foo { val: 21 };
    foo.foo();
    //assert_eq!(foo.bar(), 21);
    miri_unsafe_aliasing();
    no_black_box();
    with_black_box();
}
