use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, SomeTrait};

trait SomeTrait {
    fn do_something();
}

#[derive(HelloMacro)]
struct Pancakes;

//impl HelloMacro for Pancakes {
//    fn hello_macro() {
//        println!("Hello, Macro! My name is Pancakes!");
//    }
//}

#[derive(HelloMacro, SomeTrait)]
enum FooBar {
    Foo,
    Bar,
    Baz,
    Quux,
}

fn main() {
    Pancakes::hello_macro();
    FooBar::hello_macro();
    FooBar::do_something();
}