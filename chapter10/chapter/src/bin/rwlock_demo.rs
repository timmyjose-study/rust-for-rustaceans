use std::sync::{Arc, RwLock};
use std::thread;

// a mutex has totally exclusive access to the critical section whereas an rwlock allows multiple
// readers but a single reader access to the critical section.

fn main() {
    let data = Arc::new(RwLock::new(5));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let data = data.read().unwrap();
            println!("Thread {:?} sees value {}", thread::current().id(), *data);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        let mut data_clone = data_clone.write().unwrap();
        *data_clone += 100;
    })
    .join()
    .unwrap();

    println!("After exclusive writing, data = {}", data.read().unwrap());
}
