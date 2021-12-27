use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static X: AtomicBool = AtomicBool::new(false);
static Y: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut handles = Vec::new();

    handles.push(thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        let r1 = Y.load(Ordering::Relaxed);
        X.store(r1, Ordering::Relaxed);
    }));

    handles.push(thread::spawn(|| {
        let r2 = X.load(Ordering::Relaxed);
        if r2 {
            panic!("r2 is inexplicably true!");
        }
        Y.store(true, Ordering::Relaxed);
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}
