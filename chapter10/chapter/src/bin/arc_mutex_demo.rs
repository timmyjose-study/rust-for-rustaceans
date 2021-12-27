use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 20;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();

    for i in 1..=N {
        let (data, tx) = (Arc::clone(&data), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += i;

            // send the data only when the final total has been reached
            if i == N {
                tx.send(*data).unwrap();
            }
        });
    }

    println!("Sum of numbers from 1 to {} = {}", N, rx.recv().unwrap());
}
