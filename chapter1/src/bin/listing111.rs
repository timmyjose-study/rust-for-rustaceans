//! variance and lifetimes

struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn main() {
    let mut s = "hello"; // 'a
    *MutStr { s: &mut s }.s = "world"; // 'b
    println!("{}", s);
}
