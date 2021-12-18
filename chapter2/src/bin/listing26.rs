use std::fmt;
use std::iter::IntoIterator;
use std::slice::Iter;

struct MyIterable {
    elements: Vec<i32>,
}

impl MyIterable {
    pub fn new(elements: Vec<i32>) -> Self {
        MyIterable { elements }
    }
}

impl IntoIterator for MyIterable {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a> IntoIterator for &'a MyIterable {
    type Item = &'a i32;
    type IntoIter = Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl fmt::Debug for MyIterable
where
    for<'a> &'a Self: IntoIterator,
    for<'a> <&'a Self as IntoIterator>::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_list().entries(self).finish()
    }
}

fn main() {
    let my_iterable = MyIterable::new(vec![1, 2, 3, 4, 5]);
    for e in &my_iterable {
        println!("{:?}", *e);
    }

    // this is due to the generic impl Debug for MyIterable
    println!("{:?}", my_iterable);

    for e in &my_iterable {
        println!("{:?}", e);
    }
}
