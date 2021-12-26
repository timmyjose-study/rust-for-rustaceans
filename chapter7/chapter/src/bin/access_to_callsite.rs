macro_rules! please_set {
    ($i:ident, $e:expr) => {
        $i = $e;
    };
}

fn main() {
    let mut x = 1;
    please_set!(x, x + 1);
    assert_eq!(x, 2);
}
