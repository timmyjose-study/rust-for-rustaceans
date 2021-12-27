use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    let _ = thread::spawn(move || {
        let _guard = lock.lock().unwrap();
        panic!(); // poison the mutex
    })
    .join();

    // accessing the poisoned mutex' guard is still possible
    let mut guard = match lock2.lock() {
        Err(poisoned_err) => poisoned_err.into_inner(),
        Ok(guard) => guard,
    };

    println!("{}", *guard);
    *guard += 100;
    println!("{}", guard);
}
