use std::error::Error;

type ErrorT = Box<dyn Error + Send + Sync + 'static>;

mod errors {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct FooError;

    impl Error for FooError {}

    impl fmt::Display for FooError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "FooError")
        }
    }

    #[derive(Debug)]
    pub struct BarError {
        pub kind: BarErrorKind,
    }

    #[derive(Debug)]
    pub enum BarErrorKind {
        SomethingAwful,
        NotTooBad,
        GiveUpAlready,
    }

    impl fmt::Display for BarErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let text = match *self {
                BarErrorKind::SomethingAwful => "Something awful happened",
                BarErrorKind::NotTooBad => "Maybe there's still hope!",
                BarErrorKind::GiveUpAlready => "Switch off and go home",
            };
            write!(f, "{}", text)
        }
    }

    impl Error for BarError {}

    impl fmt::Display for BarError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }
}

fn generate_foo_error() -> ErrorT {
    Box::new(errors::FooError {})
}

fn generate_bar_error_one() -> ErrorT {
    Box::new(errors::BarError {
        kind: errors::BarErrorKind::SomethingAwful,
    })
}

fn generate_bar_error_two() -> ErrorT {
    Box::new(errors::BarError {
        kind: errors::BarErrorKind::NotTooBad,
    })
}

fn generate_bar_error_three() -> ErrorT {
    Box::new(errors::BarError {
        kind: errors::BarErrorKind::GiveUpAlready,
    })
}
fn handle_any_error(err: ErrorT) {
    if let Some(_) = err.downcast_ref::<errors::FooError>() {
        println!("Got a FooError!");
    } else if let Some(e) = err.downcast_ref::<errors::BarError>() {
        println!("Got a Bar Error: {}", e);
    }
}

fn main() {
    handle_any_error(generate_foo_error());
    handle_any_error(generate_bar_error_one());
    handle_any_error(generate_bar_error_two());
    handle_any_error(generate_bar_error_three());
}
