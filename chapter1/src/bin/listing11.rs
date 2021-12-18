#![allow(unused_variables)]
#![allow(unused_assignments)]
fn main() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
}