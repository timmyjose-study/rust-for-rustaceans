#[repr(C)]
struct Foo {
    tiny: bool,  // 1
    normal: u32, // 4
    small: u8,   // 1
    long: u64,   // 8
    short: u16,  // 2
}

struct Bar {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}

struct Inch(i32);

#[repr(transparent)]
struct InchAgain(i32);

fn main() {
    println!("sizeof Foo = {} bytes", std::mem::size_of::<Foo>());
    println!("sizeof Bar = {} bytes", std::mem::size_of::<Bar>());
    println!("sizeof Inch = {} bytes", std::mem::size_of::<Inch>());
    println!(
        "sizeof InchAgain = {} bytes",
        std::mem::size_of::<InchAgain>()
    );
}