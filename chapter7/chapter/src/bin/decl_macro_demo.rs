trait DoSomething {
    fn do_something();
}

#[macro_export]
macro_rules! impl_do_something {
    ($($t:ty as $name:ident,)*) => {
        impl_do_something!($($t as $name),*); // this matches the second rule
    };

    ($($t:ty as $name:ident),*) => {
        $(impl DoSomething for $t {
            fn do_something() {
                println!("Doing something for {}", stringify!($name));
            }
        })*
    };
}

struct Foo;

struct Bar(i32);

enum Baz {
    Quux,
    FooBar,
}

impl_do_something! {
    Foo as Foo,
    Bar as Bar,
    Baz as Baz,
    i32 as i32,
    f64 as f64,
}

fn main() {
    i32::do_something();
    Foo::do_something();
    Bar::do_something();
    Baz::do_something();
    f64::do_something();
}