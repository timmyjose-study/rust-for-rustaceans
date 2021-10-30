fn replace_with_84(s: &mut Box<i32>) {
    // std::mem::take is the same as std::mem::replace(&mut value, Default::default())
    let was = std::mem::take(s);
    *s = was;

    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
}

fn main() {
    let mut s = Box::new(42);
    assert_eq!(*s, 42);
    replace_with_84(&mut s);
    assert_eq!(*s, 84);
}