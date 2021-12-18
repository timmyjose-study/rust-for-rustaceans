trait Pattern<'a> {
    fn is_contained_in(&self, haystack: &'a str) -> bool;
}

#[repr(transparent)]
struct MyString(String);

impl MyString {
    // static dispatch
    pub fn contains<'b, 'a: 'b>(&'a self, p: impl Pattern<'b>) -> bool {
        p.is_contained_in(&self.0)
    }

    // dynamic dispatch
    pub fn also_contains<'b, 'a: 'b>(&'a self, p: &'b dyn Pattern<'b>) -> bool {
        p.is_contained_in(&self.0)
    }
}

impl<'a> Pattern<'a> for &'a str {
    fn is_contained_in(&self, haystack: &'a str) -> bool {
        haystack.contains(self)
    }
}

fn main() {
    let text = MyString("hello, world".to_string());

    println!("{}", text.contains("ello"));
    println!("{}", text.contains("xllo"));

    println!("{}", text.also_contains(&"ello"));
    println!("{}", text.also_contains(&"xllo"));
}
