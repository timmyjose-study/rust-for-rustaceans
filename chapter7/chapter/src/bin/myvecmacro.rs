#[macro_export]
macro_rules! myvec {
    ( $($x:expr),* ) => {{
        let mut v = Vec::new();

        $(
            v.push($x);
        )*
        v
    }};
}

fn main() {
    let v1 = myvec!["hello", "world", "nice", "to", "meet", "you"];
    for e in v1 {
        print!("{} ", e);
    }
    println!();

    let v2 = myvec![1, 2, 3, 4, 5];
    assert_eq!(15, v2.iter().sum::<i32>());
}