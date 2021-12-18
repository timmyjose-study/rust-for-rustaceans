use std::sync::Arc;

trait Foo {}

// size of wide ptr = 2 x size of usize on the platform.
fn main() {
    println!("size of usize = {} bytes", std::mem::size_of::<usize>());

    println!(
        "size of dyn Foo = {} bytes",
        std::mem::size_of::<&dyn Foo>()
    );

    println!(
        "size of Box<Foo> = {} bytes",
        std::mem::size_of::<Box<dyn Foo>>()
    );

    println!(
        "size of Arc<Foo> = {} bytes",
        std::mem::size_of::<Arc<dyn Foo>>()
    );
}
