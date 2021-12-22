use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct BasicError;

impl Error for BasicError {}

impl fmt::Display for BasicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BasicError")
    }
}

fn generate_error() -> Result<(), Box<dyn Error>> {
    Err(Box::new(BasicError {}))
}

fn handle_error() {
    match generate_error() {
        Ok(_) => println!("Okay"),
        Err(e) => {
            if e.downcast_ref::<BasicError>().is_some() {
                println!("Got a BasicError");
            } else {
                println!("Got some other error");
            }
        }
    }
}

fn main() {
    handle_error();
}