fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let old_v = std::mem::take(&mut v); // replaces source with an empty value
    assert_eq!(old_v, vec![1, 2, 3, 4, 5]);
    assert!(v.is_empty());
}
