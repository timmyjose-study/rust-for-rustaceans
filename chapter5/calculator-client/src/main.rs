use calculator::{add, sub};
use std::error::Error;
use std::io::{self, Write};

fn get_num() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().parse::<i32>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("Enter the first number: ");
    io::stdout().flush()?;
    let first = get_num();

    print!("Enter the second number: ");
    io::stdout().flush()?;
    let second = get_num();

    println!("{} + {} = {}", first, second, add(first, second));
    println!("{} - {} = {}", first, second, sub(first, second));
    println!(
        "{} * {} = {}",
        first,
        second,
        calculator::mul(first, second)
    );
    println!(
        "{} / {} = {}",
        first,
        second,
        calculator::div(first, second)
    );

    Ok(())
}
