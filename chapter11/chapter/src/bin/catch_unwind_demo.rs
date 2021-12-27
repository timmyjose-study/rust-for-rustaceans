fn panic_unconditionally() {
    println!("Hello from panic_unconditionally");
    panic!();
}

fn main() {
    let result = std::panic::catch_unwind(|| {
        panic_unconditionally();
    });
    assert!(result.is_err());
    println!("Adios from main!");
}