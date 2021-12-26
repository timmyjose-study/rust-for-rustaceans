use std::default::Default;
use std::ops::SubAssign;

struct SomeType<T> {
    some_size: T,
}

impl<T> SomeType<T>
where
    T: SubAssign + Default,
{
    pub unsafe fn decr(&mut self) {
        self.some_size -= T::default();
    }
}

fn main() {
    let mut st = SomeType { some_size: 42i32 };
    unsafe {
        st.decr();
    }
    println!("{}", st.some_size);
}