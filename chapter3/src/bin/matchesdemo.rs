fn main() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(42);
    assert!(matches!(bar, Some(x) if x % 2 == 0));
}