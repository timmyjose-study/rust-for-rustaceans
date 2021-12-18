#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

fn main() {
    let x1 = 42;
    let y1 = Box::new(84);

    {
        let z = (x1, y1);
    } // x1 and y1 are dropped here

    let x2 = x1; // this is fine since x1 is `Copy`
    assert_eq!(x2, 42);
}
