struct SomeType<T> {
    ptr: *const T,
}

impl<T> SomeType<T> {
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

fn main() {
    let t = 42i32;
    let st = SomeType {
        ptr: &t as *const i32,
    };
    let ptr = st.as_ref();
    println!("{}", *ptr);
}
