#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // uninitialised, no flow to next line
    let mut x;
    //assert_eq!(x, 42);
    x = 42;
    let y = &x;
    // this is fine since the shared flow ends
    // here
    assert_eq!(*y, 42);
    x = 43;
    // exclusive access and shared access flows at this point,
    // so this is a problem.
    //assert_eq!(*y, 42);
}
