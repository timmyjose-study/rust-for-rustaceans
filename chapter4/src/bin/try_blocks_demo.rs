#![feature(try_blocks)]

use std::num::ParseIntError;

fn do_something_good() -> Result<i32, ParseIntError> {
    let one = "1".parse::<i32>()?;
    let two = "2".parse::<i32>()?;
    let three = "3".parse::<i32>()?;
    println!("cleaning up");
    Ok(one + two + three)
}

fn do_something_bad() -> Result<i32, ParseIntError> {
    let one = "1".parse::<i32>()?;
    let two = "foo".parse::<i32>()?;
    let three = "3".parse::<i32>()?;
    println!("cleaning up"); // this will never run
    Ok(one + two + three)
}

fn do_something_bad_fixed() -> Result<i32, ParseIntError> {
    let res = try {
        let one = "1".parse::<i32>()?;
        let two = "foo".parse::<i32>()?;
        let three = "3".parse::<i32>()?;
        one + two + three
    };
    println!("cleaning up"); // this will always be run
    res
}
fn main() {
    let result: Result<i32, ParseIntError> =
        try { "1".parse::<i32>()? + "2".parse::<i32>()? + "3".parse::<i32>()? };
    assert_eq!(result, Ok(6));

    let result: Result<i32, ParseIntError> =
        try { "1".parse::<i32>()? + "foo".parse::<i32>()? + "3".parse::<i32>()? };
    assert!(result.is_err());

    assert_eq!(do_something_good(), Ok(6));
    assert!(do_something_bad().is_err());
    assert!(do_something_bad_fixed().is_err());
}
