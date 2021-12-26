macro_rules! let_foo {
    ($x:expr) => {
        let foo = $x;
    };
}

macro_rules! defun {
    ($name: ident) => {
        fn $name() {
            println!("Hello!");
        }
    };
}

fn bar() {
    println!("Hello from bar");
}

fn main() {
    let foo = 1;
    let_foo!(2);
    assert_eq!(foo, 1); // hygiene at work

    // this will not work since
    // hygiene only applies to variable names
    // and this will overwrite the manually `bar` defined
    // above
    bar();
    defun!(bar);
    bar();
}
