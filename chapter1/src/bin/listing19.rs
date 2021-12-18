fn main() {
    let mut x = Box::new(42);
    let mut z = &x; // 'a

    for i in 0..100 {
        println!("{}", z); // 'a still active
        x = Box::new(i);
        z = &x; // 'a restarts
    }
    println!();
    println!("{}", z);
}
