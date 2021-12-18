fn main() {
    let mut s = 0;
    let r = 42;
    // shared references are `Copy`
    cache(&r, &mut s);
}

fn cache(input: &i32, sum: &mut i32) {
    *sum += *input + *input;
    // rust can assume that this is always true, barring `unsafe`
    assert_eq!(*sum, 2 * *input);
}
