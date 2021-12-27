use std::sync::Arc;
use std::thread;

fn sharing_immutable_data() {
    let five = Arc::new(5);
    let mut handles = Vec::new();

    for _ in 0..10 {
        let five = Arc::clone(&five);
        handles.push(thread::spawn(move || {
            println!(
                "strong count = {}, weak count = {}",
                Arc::strong_count(&five),
                Arc::weak_count(&five)
            );
            println!("{}", five);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn sharing_mutable_data() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let val = Arc::new(AtomicUsize::new(5));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let val = Arc::clone(&val);
        handles.push(thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    sharing_immutable_data();
    sharing_mutable_data();
}
