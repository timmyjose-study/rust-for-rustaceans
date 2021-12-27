use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn some_long_calculation() -> usize {
    thread::sleep(Duration::from_millis(5));
    42
}

fn another_long_calculation() -> usize {
    thread::sleep(Duration::from_millis(2));
    5
}

fn main() {
    let (tx, rx) = channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send(another_long_calculation()).unwrap();
        tx1.send(another_long_calculation()).unwrap();
        tx1.send(another_long_calculation()).unwrap();
        tx1.send(another_long_calculation()).unwrap();
        tx1.send(another_long_calculation()).unwrap();
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        tx2.send(some_long_calculation()).unwrap();
        tx2.send(some_long_calculation()).unwrap();
        tx2.send(some_long_calculation()).unwrap();
        tx2.send(some_long_calculation()).unwrap();
        tx2.send(some_long_calculation()).unwrap();
    });

    thread::spawn(move || {
        tx.send(another_long_calculation()).unwrap();
        tx.send(some_long_calculation()).unwrap();
    });

    println!("Main doing some work");

    while let Ok(val) = rx.recv() {
        println!("Received {:?}", val);
    }

    println!("Adios from main!");
}
