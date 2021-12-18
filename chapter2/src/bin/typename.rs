use std::any::{type_name, Any};

fn print_typename<T: Any>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let x = 42;
    let s = "hello";

    print_typename(&x);
    print_typename(&s);

    let x1 = &32;
    print_typename(&x1);
}
