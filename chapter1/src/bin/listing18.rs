fn main() {
    let mut x = Box::new(42);
    // 'a starts here
    let r = &x; // &mut Box<i32>

    let rand = 0.8;

    if rand > 0.5 {
        *x = 84; // this works since Box<T> is an owning pointer, so &x or &mut x doesn't matter here
        println!("{}", x);
    } else {
        println!("{}", r); // 'a ends here
    }
}
