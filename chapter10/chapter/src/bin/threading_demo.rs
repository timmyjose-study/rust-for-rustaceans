use std::thread;
use std::time::Duration;

fn say_hello() {
    thread::sleep(Duration::from_millis(5000));
    println!("Hello from thread!");
}

fn main() {
    let handle = thread::spawn(|| {
        say_hello();
    });

    handle.join().unwrap();
    println!("Adios from main!");
}