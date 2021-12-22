mod foo {
    pub struct Unit;

    pub trait Foo1 {
        fn foo(&self);
    }

    impl Foo1 for Unit {
        fn foo(&self) {
            println!("Foo1 says hi!");
        }
    }
}

mod bar {
    use super::foo::Unit;

    pub trait Foo2 {
        fn foo(&self);
    }

    impl Foo2 for Unit {
        fn foo(&self) {
            println!("Foo2 says hi!");
        }
    }
}

fn main() {
    use bar::*;
    use foo::*;

    let u = Unit;
    //u.foo();
    Foo1::foo(&u);
    Foo2::foo(&u);
}
