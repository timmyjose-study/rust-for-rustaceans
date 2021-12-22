fn main() {
    let s1 = "Hello, world";
    let r1 = do_something(&s1);
    println!("{:?}", r1.as_ref());

    let s2 = "Hola, mundo!".to_string();
    let r2 = do_something(&s2);
    println!("{:?}", r2.as_ref());

    let s3 = Box::new("Privet, Mir!");
    let r3 = do_something(&*s3);
    println!("{:?}", r3.as_ref());
}

fn do_something(input: &dyn AsRef<str>) -> &dyn AsRef<str> {
    input
}
