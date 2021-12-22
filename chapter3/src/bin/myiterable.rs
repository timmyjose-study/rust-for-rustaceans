use std::iter::IntoIterator;

#[derive(Debug)]
struct MyVecWrapper<T>(Vec<T>);

impl<T> MyVecWrapper<T> {
    pub fn new(wrapped: Vec<T>) -> Self {
        MyVecWrapper(wrapped)
    }
}

impl<T> IntoIterator for MyVecWrapper<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a MyVecWrapper<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut MyVecWrapper<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

fn main() {
    let mut vi = MyVecWrapper::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", vi);

    for e in &vi {
        print!("{} ", *e);
    }
    println!();

    for e in &mut vi {
        *e += 10;
    }

    for e in vi {
        print!("{} ", e);
    }
    println!();

    let mut vs = MyVecWrapper(vec![
        "hello".to_string(),
        "world".to_string(),
        "nice".to_string(),
        "to".to_string(),
        "meet".to_string(),
        "you".to_string(),
    ]);
    println!("{:?}", vs);

    for e in &vs {
        print!("{} ", e);
    }
    println!();

    for e in &mut vs {
        e.push('!');
    }

    for e in vs {
        print!("{} ", e);
    }
    println!();
}