trait Foo {
    fn quux() {
        println!("Foo says Quux!");
    }

    fn foobar(&self) {
        println!("Hola from foobar");
    }
}

struct Bar;

impl Foo for Bar {}

fn invoke_quux<T: Foo>(_val: &T) {
    T::quux()
}

fn main() {
    let b = Bar;
    invoke_quux(&b);
    <Bar as Foo>::quux();

    b.foobar();
    Foo::foobar(&b);
}
