// box leak is useful in order to share a common resource,
// which must live for the duration of the program. Dropping the reference is
// a memory leak.

fn main() {
    let x = Box::new(42);
    let xref: &'static usize = Box::leak(x);
    foo(xref);
    bar(xref);
}

fn foo(r: &'static usize) {
    println!("Inside foo, x = {}", *r);
}

fn bar(r: &'static usize) {
    println!("Inside bar, x = {}", *r);
}
