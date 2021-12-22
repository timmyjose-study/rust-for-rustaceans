use std::fmt::Debug;

trait Bar: Debug {}

#[derive(Debug)]
struct Foo {
    x: i32,
}

impl Foo {
    pub fn new(x: i32) -> Self {
        Foo { x }
    }
}

impl Bar for Foo {}

fn do_something<T: Bar + Debug + ?Sized>(input: &T) {
    println!("[do_something] input = {:?}", input);
}

fn do_something_again(input: &dyn Bar) {
    println!("[do_something_again] input = {:?}", input);
}

fn main() {
    let foo = Foo::new(42);

    do_something(&foo);
    do_something(&foo as &dyn Bar);

    do_something_again(&foo);
    do_something_again(&foo as &dyn Bar);
}
