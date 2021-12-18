fn main() {
    let r = 42;
    let mut s = 0;
    noalias(&r, &mut s);
    assert_eq!(s, 3);
}

// due to exclusive access of borrowed references (mutable references),
// we know that `input` and `output` will never be the same, and hence
// the code can be optimised into a simple `if-else` chain.
fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }

    if *input != 1 {
        *output = 3;
    }
}
