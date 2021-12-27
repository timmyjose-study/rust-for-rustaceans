use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel(); // 5 messages buffer limit

    {
        // this scope is necessary to drop tx0 so that the loop over
        // `recv` actually finishes.

        let tx0 = tx.clone();
        thread::sleep(Duration::from_millis(1));
        tx0.send(0).unwrap();
        thread::sleep(Duration::from_millis(1));
        tx0.send(0).unwrap();
        thread::sleep(Duration::from_millis(1));
        tx0.send(0).unwrap();
        thread::sleep(Duration::from_millis(1));
        tx0.send(0).unwrap();
    }

    let tx1 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(3));
        tx1.send(1).unwrap();
        tx1.send(2).unwrap();
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        tx2.send(3).unwrap();
        tx2.send(4).unwrap();
        tx2.send(5).unwrap();
    });

    let tx3 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2));
        tx3.send(6).unwrap();
    });

    let tx4 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        tx4.send(7).unwrap();
        tx4.send(8).unwrap();
        tx4.send(9).unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        tx.send(10).unwrap();
    });

    println!("Hello from main!");

    while let Ok(val) = rx.recv() {
        println!("Received {:?}", val);
    }

    println!("Adios from main!");
}
