#![feature(auto_traits)]
#![feature(negative_impls)]

auto trait Foo {}

struct Bar;

struct Quux {
    x: i32,
}

impl !Foo for Quux {}

fn check_if_foo<F: Foo>(val: &F) {
    println!("Yup, we got a foo!");
}

fn main() {
    let bar = Bar;
    check_if_foo(&bar);

    let quux = Quux { x: 42 };
    //check_if_foo(&quux);
}
