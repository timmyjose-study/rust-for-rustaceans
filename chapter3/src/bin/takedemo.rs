fn main() {
    let mut my_option = Some(42);
    let answer = my_option.take();
    assert_eq!(answer, Some(42));
    assert!(my_option.is_none());
}
