use std::default::Default;
use std::fmt;

#[derive(Debug)]
struct ArrayVec<T, const N: usize> {
    values: [Option<T>; N],
    len: usize,
}

impl<T: Copy + Default, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        ArrayVec {
            values: [T::default().into(); N],
            len: 0,
        }
    }

    pub fn try_push(&mut self, t: T) -> Result<(), T> {
        if self.len == N {
            return Err(t);
        }

        self.values[self.len] = Some(t);
        self.len += 1;
        Ok(())
    }
}

impl<T: fmt::Display, const N: usize> fmt::Display for ArrayVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len == 0 {
            write!(f, "{}", "[]")?;
        } else {
            let mut idx = 0;
            write!(f, "{}", "[")?;
            while idx < self.len {
                write!(f, "{},", self.values[idx].as_ref().unwrap())?;
                idx += 1;
            }
            write!(f, "{}", self.values[self.len - 1].as_ref().unwrap())?;
            write!(f, "{}", "]")?;
        }

        Ok(())
    }
}

fn main() {
    let mut arr1 = ArrayVec::<i32, 100>::new();
    assert_eq!(arr1.values.len(), 100);
    assert_eq!(arr1.len, 0);
    println!();

    println!("Debug view...");
    println!("{:?}", arr1);

    println!("Display view...");
    println!("{}", arr1);

    for n in 1..=10 {
        arr1.try_push(n).unwrap();
    }
    println!("{}", arr1);
}
