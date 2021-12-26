struct Foo<'a> {
    my_mut: &'a mut String,
}

impl<'a> Foo<'a> {
    pub fn new<'b: 'a>(m: &'b mut String) -> Self {
        Foo { my_mut: m }
    }
}

fn do_something(foo: &Foo) {
    println!("{}", *foo.my_mut);
    // reference semantics is directed by the root of the reference tree -
    // since `foo` is a borrow, the nested mutable reference cannot be
    // modified through `foo`.
    //foo.my_mut.push_str("!");
}

fn main() {
    let mut s = "Hello, world".to_string();
    let foo = Foo::new(&mut s);
    do_something(&foo);
}
