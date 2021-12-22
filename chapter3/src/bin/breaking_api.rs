struct Unit {
    field: bool,
    //another_field: i32,
}

fn is_true(u: Unit) -> bool {
    matches!(u, Unit { field: true })
}

fn main() {
    assert!(is_true(Unit { field: true }));
    assert!(!is_true(Unit { field: false }));
}
