/// Some struct named Foo
pub struct Foo {
    pub val: i32,
}

impl Foo {
    /// Does what `foo` is supposed to do!
    ///
    /// Here's an example:
    ///
    /// ```
    /// # use config_test::Foo;
    ///
    /// let foo = Foo { val: 42};
    /// foo.foo();
    /// ```
    pub fn foo(&self) {
        println!("Foo");
    }

    #[cfg(test)]
    pub(crate) fn bar(&self) -> i32 {
        self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar() {
        let foo = Foo { val: 42 };
        assert_eq!(foo.bar(), 42);
    }
}
