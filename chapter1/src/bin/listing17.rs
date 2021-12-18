//! a mutable reference has two major differences when compared to an owned value:
//! 1. the mutable reference cannot drop the value, and
//! 2. when moving the value through the mutable reference, we must put another valid
//! value inside it.

fn replace_with_84(s: &mut Box<i32>) {
    //let was = *s;
    let was = std::mem::take(s);
    *s = was;

    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_eq!(**s, 84);
    assert_eq!(*r, 100);
}

fn main() {
    let mut s = Box::new(100);
    replace_with_84(&mut s);
}
